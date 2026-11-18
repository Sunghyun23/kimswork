#pragma once
#include "ChordVoicing.h"
#include <atomic>
#include <cstdlib>
namespace donut {
struct SmoothInput {
    int start=0,count=4,active=4,page=0,scope=0;
    std::array<std::array<int,4>,16> chords{}; // root, quality, voicing, octave index
    bool operator==(const SmoothInput& b)const{return start==b.start&&count==b.count&&active==b.active&&page==b.page&&scope==b.scope&&chords==b.chords;}
};
struct SmoothPlan {
    SmoothInput input{};
    std::array<int,16> voice{},octave{},before{},after{};
    int movementBefore=0,movementAfter=0,valid=0;
};
// No raw shared structs: UI publishes atomic cells, audio accepts one consistent
// sequence or retries next block. One UI writer is protected by the idle guard.
static_assert(std::atomic<int>::is_always_lock_free,"Smooth plan cells must be lock-free");
class AtomicSmoothPlan {
    std::atomic<unsigned> sequence_{0};
    std::array<std::atomic<int>,136> cells_{};
public:
    void publish(const SmoothPlan& p){
        sequence_.fetch_add(1,std::memory_order_acq_rel);int j=0;
        auto put=[&](int v){cells_[j++].store(v,std::memory_order_relaxed);};
        put(p.input.start);put(p.input.count);put(p.input.active);put(p.input.page);put(p.input.scope);
        for(const auto& chord:p.input.chords)for(int v:chord)put(v);
        for(const auto* a:{&p.voice,&p.octave,&p.before,&p.after})for(int v:*a)put(v);
        put(p.movementBefore);put(p.movementAfter);put(p.valid);
        sequence_.fetch_add(1,std::memory_order_release);
    }
    bool read(SmoothPlan& p)const{
        const unsigned before=sequence_.load(std::memory_order_acquire);if(!before||(before&1))return false;int j=0;
        auto get=[&]{return cells_[j++].load(std::memory_order_relaxed);};
        p.input.start=get();p.input.count=get();p.input.active=get();p.input.page=get();p.input.scope=get();
        for(auto& chord:p.input.chords)for(int& v:chord)v=get();
        for(auto* a:{&p.voice,&p.octave,&p.before,&p.after})for(int& v:*a)v=get();
        p.movementBefore=get();p.movementAfter=get();p.valid=get();
        std::atomic_thread_fence(std::memory_order_acquire);
        return before==sequence_.load(std::memory_order_acquire);
    }
};
struct SmoothCandidate {std::array<int,6> notes{};int count=0,voice=0,octave=2;};
inline bool smoothCandidate(const std::array<int,4>& chord,int voice,int octave,SmoothCandidate& c){
    // Calculate unclamped notes through the same proven chord construction.
    c.count=chordNotes(48,chord[1],voice,0,c.notes);c.voice=voice;c.octave=octave;
    for(int i=0;i<c.count;++i){c.notes[i]+=chord[0]-48+12*(octave-2);if(c.notes[i]<0||c.notes[i]>127||(i&&c.notes[i]==c.notes[i-1]))return false;}
    return true;
}
struct SmoothCost {
    int top=1000000,inner=1000000,edit=1000000;
    bool operator<(const SmoothCost& b)const{return top!=b.top?top<b.top:inner!=b.inner?inner<b.inner:edit<b.edit;}
    SmoothCost operator+(const SmoothCost& b)const{return {top+b.top,inner+b.inner,edit+b.edit};}
};
inline SmoothCost smoothEdge(const SmoothCandidate& a,const SmoothCandidate& b){
    int inner=std::abs(a.notes[0]-b.notes[0]);
    for(const auto* x:{&a,&b}){const auto& y=x==&a?b:a;for(int i=0;i<x->count;++i){int nearest=128;for(int j=0;j<y.count;++j)nearest=std::min(nearest,std::abs(x->notes[i]-y.notes[j]));inner+=nearest;}}
    return {std::abs(a.notes[a.count-1]-b.notes[b.count-1]),inner,0};
}
// Only called from non-render UI polling/idle, never from process(). Fixed
// candidates and dynamic programming also keep this deterministic and bounded.
inline SmoothPlan findSmoothVoicing(const SmoothInput& in){
    SmoothPlan plan;plan.input=in;
    if(in.start<0||in.count<1||in.count>16||in.start+in.count>in.active||in.active>16)return plan;
    std::array<std::array<SmoothCandidate,30>,16> candidates{};
    std::array<int,16> counts{};std::array<SmoothCandidate,16> original{};
    for(int i=0;i<in.count;++i){
        const auto& chord=in.chords[in.start+i];
        if(!smoothCandidate(chord,chord[2],chord[3],original[i]))return plan;
        plan.before[i]=original[i].notes[original[i].count-1];
        if(i==0){candidates[0][0]=original[0];counts[0]=1;continue;}
        const int span=std::max(24,original[i].notes[original[i].count-1]-original[i].notes[0]);
        for(int voice=0;voice<6;++voice){
            if(voice>=3&&voice>chordToneCount(chord[1]))continue;
            for(int octave=0;octave<5;++octave){SmoothCandidate c;
                if(!smoothCandidate(chord,voice,octave,c)||c.notes[c.count-1]-c.notes[0]>span)continue;
                candidates[i][counts[i]++]=c;
            }
        }
        if(!counts[i])return plan;
    }
    std::array<std::array<SmoothCost,30>,16> scores{};
    std::array<std::array<int,30>,16> previous{};scores[0][0]={0,0,0};
    for(int i=1;i<in.count;++i)for(int j=0;j<counts[i];++j){
        const auto& c=candidates[i][j];const auto& o=original[i];
        const int edit=std::abs(c.notes[c.count-1]-o.notes[o.count-1])+std::abs(c.notes[0]-o.notes[0])+2*(c.voice!=o.voice)+2*std::abs(c.octave-o.octave);
        for(int k=0;k<counts[i-1];++k){auto score=scores[i-1][k]+smoothEdge(candidates[i-1][k],c);score.edit+=edit;
            if(score<scores[i][j]){scores[i][j]=score;previous[i][j]=k;}}
    }
    SmoothCost best;int last=0;
    for(int j=0;j<counts[in.count-1];++j){const auto total=scores[in.count-1][j]+smoothEdge(candidates[in.count-1][j],candidates[0][0]);if(total<best){best=total;last=j;}}
    for(int i=in.count-1;i>=0;--i){const auto& c=candidates[i][last];plan.voice[i]=c.voice;plan.octave[i]=c.octave;plan.after[i]=c.notes[c.count-1];last=previous[i][last];}
    for(int i=0;i<in.count;++i)plan.movementBefore+=std::abs(plan.before[i]-plan.before[(i+1)%in.count]);
    plan.movementAfter=best.top;plan.valid=1;return plan;
}
}
