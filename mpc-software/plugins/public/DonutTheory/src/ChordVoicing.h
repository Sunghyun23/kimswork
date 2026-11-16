#pragma once
#include <algorithm>
#include <array>
namespace donut {
inline int chordToneCount(int quality){
    static constexpr int counts[]={4,4,5,5,4,5,5,3,3,3,5};
    return counts[std::clamp(quality,0,10)];
}
// Indices 0/1/2 retain Root/First/Open; 3/4/5 add second/third/fourth inversion.
// Inversions put the chosen chord member in the bass, including extended chords.
inline int chordNotes(int root,int quality,int voicing,int octave,std::array<int,6>& notes){
    static constexpr int intervals[11][5]={{0,3,7,10,0},{0,4,7,11,0},{0,3,7,10,14},{0,4,7,11,14},{0,4,7,10,0},{0,4,7,10,14},{0,4,10,14,21},{0,2,7,0,0},{0,3,7,0,0},{0,4,7,0,0},{0,4,7,10,13}};
    quality=std::clamp(quality,0,10);voicing=std::clamp(voicing,0,5);octave=std::clamp(octave,-2,2);
    const int count=chordToneCount(quality),inversion=voicing==1?1:voicing>=3?std::min(voicing-1,count-1):0;
    const int bass=intervals[quality][inversion];
    for(int i=0;i<count;++i){
        int interval=intervals[quality][i];
        if(interval<bass)interval+=12*((bass-interval+11)/12);
        if(voicing==2&&i%2)interval+=12;
        notes[i]=std::clamp(root+12*octave+interval,0,127);
    }
    std::sort(notes.begin(),notes.begin()+count); // Spread plays from bass upwards.
    return count;
}
}
