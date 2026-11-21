// Original kimswork chord mathematics, shared from Donut Theory; MIT.
#pragma once
namespace felt {
static constexpr int kTypeCount=32;
static constexpr int kTypeSizes[32]={4,4,5,5,4,5,5,3,3,3,5,4,4,5,5,5,5,5,5,4,5,5,5,5,5,4,4,4,4,4,4,3};
static constexpr int kTypeIntervals[32][5]={{0,3,7,10,0},{0,4,7,11,0},{0,3,7,10,14},{0,4,7,11,14},{0,4,7,10,0},{0,4,7,10,14},{0,4,10,14,21},{0,2,7,0,0},{0,3,7,0,0},{0,4,7,0,0},{0,4,7,10,13},{0,3,7,9,0},{0,4,7,9,0},{0,3,7,9,14},{0,4,7,9,14},{0,3,10,14,17},{0,3,10,14,21},{0,4,11,14,18},{0,4,11,14,21},{0,5,7,10,0},{0,5,7,10,14},{0,5,10,14,21},{0,4,7,10,15},{0,4,7,10,20},{0,4,10,13,18},{0,3,6,10,0},{0,3,6,9,0},{0,4,8,10,0},{0,3,7,11,0},{0,4,7,14,0},{0,3,7,14,0},{0,5,7,0,0}};
static const char*const kTypeSuffixes[32]={"m7","maj7","m9","maj9","7","9","13","sus2","m","","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"};
inline bool minorType(int q){switch(q){case 0:case 2:case 8:case 11:case 13:case 15:case 16:case 25:case 26:case 28:case 30:return true;default:return false;}}
}
