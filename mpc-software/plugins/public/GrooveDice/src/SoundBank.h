// Original GDBANK01 reader. No third-party packed-library decoder or DRM code.
#pragma once
#include "dsp/wav.h"
#include <array>
#include <cstring>
#include <fstream>
namespace groove {
struct SoundBank {
 std::array<dsp::WavData,64> sounds;
 std::array<std::array<char,65>,64> ids{};
 struct PatternNote{int tick=0,role=0,velocity=100,gate=80;};
 std::array<PatternNote,128> pattern{};int patternCount=0,patternLength=3840;
 bool loaded=false;
 static uint32_t u32(const std::vector<uint8_t>& b,size_t p){return uint32_t(b[p])|(uint32_t(b[p+1])<<8)|(uint32_t(b[p+2])<<16)|(uint32_t(b[p+3])<<24);}
 static uint32_t checksum(const uint8_t* p,size_t n){uint32_t h=2166136261u;for(size_t i=0;i<n;++i){h^=p[i];h*=16777619u;}return h;}
 bool parse(const std::vector<uint8_t>& b){
  loaded=false;patternCount=0;
  if(b.size()<32||b.size()>256*1024*1024||std::memcmp(b.data(),"GDBANK01",8)!=0||u32(b,8)!=1||u32(b,12)!=64||u32(b,16)!=32||u32(b,20)!=96)return false;
  size_t dataStart=u32(b,24);if(dataStart!=32+64*96||size_t(u32(b,28))!=b.size()-dataStart)return false;
  // Validate every record before allocating decoded audio.
  for(int i=0;i<64;++i){size_t p=32+i*96;uint32_t role=u32(b,p+64),rate=u32(b,p+68),channels=u32(b,p+72),frames=u32(b,p+76),offset=u32(b,p+80),bytes=u32(b,p+84);
   if(role!=uint32_t(i/8)||rate<8000||rate>192000||channels<1||channels>2||frames<2||frames>1920000||offset>b.size()-dataStart||bytes>b.size()-dataStart-offset||bytes<44)return false;
   if(checksum(b.data()+dataStart+offset,bytes)!=u32(b,p+88))return false;
   for(int c=0;c<64;++c)if(!((b[p+c]>='0'&&b[p+c]<='9')||(b[p+c]>='a'&&b[p+c]<='f')))return false;
  }
  for(int i=0;i<64;++i){size_t p=32+i*96,offset=u32(b,p+80),bytes=u32(b,p+84);std::vector<uint8_t> wav(b.begin()+dataStart+offset,b.begin()+dataStart+offset+bytes);
   if(!dsp::parseWav(wav,sounds[i])||sounds[i].numFrames()!=u32(b,p+76)||sounds[i].numChannels()!=int(u32(b,p+72))||sounds[i].sampleRate!=u32(b,p+68))return false;
   std::memcpy(ids[i].data(),b.data()+p,64);ids[i][64]=0;
  }
  size_t end=0;for(int i=0;i<64;++i){size_t p=32+i*96;end=std::max(end,size_t(u32(b,p+80))+u32(b,p+84));}
  size_t p=dataStart+end;
  if(p<b.size()){
   if(b.size()-p<24||std::memcmp(b.data()+p,"GDPATT01",8)!=0||u32(b,p+8)!=1||u32(b,p+12)!=1)return false;
   uint32_t length=u32(b,p+16),count=u32(b,p+20);if(length<960||length>15360||count>128||b.size()-p!=24+count*16)return false;
   for(uint32_t i=0;i<count;++i){size_t q=p+24+i*16;uint32_t tick=u32(b,q),role=u32(b,q+4),velocity=u32(b,q+8),gate=u32(b,q+12);if(tick>=length||role>=8||velocity<1||velocity>127||gate<5||gate>500)return false;pattern[i]={int(tick),int(role),int(velocity),int(gate)};}
   patternCount=int(count);patternLength=int(length);
  }
  loaded=true;return true;
 }
 bool load(const std::string& path){
  // Read only the v1 prefix: the complete indexed library is appended, never resident in RAM.
  std::ifstream f(path,std::ios::binary);std::vector<uint8_t>h(32);if(!f.read(reinterpret_cast<char*>(h.data()),32))return false;
  if(std::memcmp(h.data(),"GDBANK01",8))return false;size_t size=size_t(u32(h,24))+u32(h,28);if(size<32||size>256*1024*1024)return false;
  std::vector<uint8_t>b(size);std::memcpy(b.data(),h.data(),32);if(!f.read(reinterpret_cast<char*>(b.data()+32),size-32))return false;return parse(b);
 }
};
}
