// Generated before first release. Append IDs after release; never reorder.
enum Key { kMaster, kTone, kBody, kCharacter3, kCharacter4, kAttack, kDecay, kSustain, kRelease, kRate, kDepth, kDrift, kWidth, kDrive, kChorus, kDelay, kRoom, kModel, kFamily, kPreset, kPresetPrev, kPresetNext, kMotionMode, kSync, kShape, kDelayDivision, kDelayFeedback, kRoomTime, kRoomDamp, kHammer, kVelocity, kResonance, kNoise, kPick, kDetune, kTuning, kPedal, kPitchRange, kVelocityCurve, kKeyOff, kMotionDelay, kMotionFade, kDriftRate, kChorusRate, kChorusDepth, kRoll, kUndo, kUnlockAll, kPanic, kInputMode, kKey, kScale, kTranspose, kStrum, kStrumDirection, kStrumVelocity, kChordGlide, kGlide, kSlots, kPage, kPagePrev, kPageNext, kSlotsAdd, kSlotsRemove, kSmooth, kChordPrev, kChordNext, kChordPreset1, kChordPreset2, kChordPreset3, kChordPreset4, kRoot1, kType1, kVoicing1, kOctave1, kPadNote1, kRoot2, kType2, kVoicing2, kOctave2, kPadNote2, kRoot3, kType3, kVoicing3, kOctave3, kPadNote3, kRoot4, kType4, kVoicing4, kOctave4, kPadNote4, kRoot5, kType5, kVoicing5, kOctave5, kPadNote5, kRoot6, kType6, kVoicing6, kOctave6, kPadNote6, kRoot7, kType7, kVoicing7, kOctave7, kPadNote7, kRoot8, kType8, kVoicing8, kOctave8, kPadNote8, kRoot9, kType9, kVoicing9, kOctave9, kPadNote9, kRoot10, kType10, kVoicing10, kOctave10, kPadNote10, kRoot11, kType11, kVoicing11, kOctave11, kPadNote11, kRoot12, kType12, kVoicing12, kOctave12, kPadNote12, kRoot13, kType13, kVoicing13, kOctave13, kPadNote13, kRoot14, kType14, kVoicing14, kOctave14, kPadNote14, kRoot15, kType15, kVoicing15, kOctave15, kPadNote15, kRoot16, kType16, kVoicing16, kOctave16, kPadNote16, kHold1, kLearn1, kChordName1, kEdit0, kEdit1, kEdit2, kEdit3, kHold2, kLearn2, kChordName2, kEdit4, kEdit5, kEdit6, kEdit7, kHold3, kLearn3, kChordName3, kEdit8, kEdit9, kEdit10, kEdit11, kHold4, kLearn4, kChordName4, kEdit12, kEdit13, kEdit14, kEdit15, kCharacter3Label, kCharacter4Label, kCurrentChordPreset, kPageInfo, kLearnInfo, kLockTone, kLockBody, kLockCharacter3, kLockCharacter4, kLockAttack, kLockDecay, kLockSustain, kLockRelease, kLockRate, kLockDepth, kLockDrift, kLockWidth, kLockDrive, kLockChorus, kLockDelay, kLockRoom, kLockModel, kLockMotionMode, kLockSync, kLockShape, kLockDelayDivision, kLockDelayFeedback, kLockRoomTime, kLockRoomDamp, kLockHammer, kLockVelocity, kLockResonance, kLockNoise, kLockPick, kLockDetune, kLockVelocityCurve, kLockKeyOff, kLockMotionDelay, kLockMotionFade, kLockDriftRate, kLockChorusRate, kLockChorusDepth, kChordEditing, kMainQ0, kMainQ1, kMainQ2, kMainQ3, kMainQ4, kMainQ5, kMainQ6, kMainQ7, kMainQ8, kMainQ9, kMainQ10, kMainQ11, kMainQ12, kMainQ13, kMainQ14, kMainQ15, kHiss, kLockHiss, kCommandPads, kCmdPrevNote, kCmdNextNote, kCmdDiceNote, kCommandLearn, kCommandInfo, kParamCount };
inline void declareParameters(mpc::Plugin& p){
{auto v=mpc::floatParam("Master","Master",-36,0,-9,"dB",1);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Tone","Tone",0,100,48,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Body","Body",0,100,62,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Character3","Character3",0,100,28,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Character4","Character4",0,100,18,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::logParam("Attack","Attack",1,1500,1,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::logParam("Decay","Decay",20,5000,650,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Sustain","Sustain",0,100,70,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::logParam("Release","Release",15,6000,420,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Rate","Motion Rate",0,1,0.5,"",3);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Depth","Motion Depth",0,100,22,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Drift","Drift",0,100,6,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Width","Width",0,100,70,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Drive","Drive",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Chorus","Chorus",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Delay","Delay",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Room","Room",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Model","Model",{"Tine","Wurli","Upright","Felt","Clav","Muted","Thumb"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Family","Family",{"EP","Piano","Pluck"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Preset","Sound Preset",{"Custom","Init","Candle Tine","Sunday Velvet","Dusty Suitcase","Afterglow","Amber Bell","Low Ceiling","Warm Current","Night Letters","Copper Wurli","Soft Static","Basement Waltz","Brick Dust","Wurli Bloom","Quiet Block","Evening Grain","Honey Circuit","Sunday Upright","Soft Floor","Loose Felt","Low Window","Paper Keys","After Hours","Dust In Light","Room Sketch","Felt Letters","Small Hours","Moss Chords","Warm Rain","Muted Ivory","Slow Bloom","Hush Piano","Quiet Tape","Dusty Clav","Copper Steps","Short Answer","Side Street","Pocket Clav","Muted Talk","Wood Circuit","Friday Bounce","Muted Thread","Short Velvet","Small Pick","Porch Light","Soft Sync","Dry Flowers","Night Pocket","Worn Strings","Thumb Notes","Little Lantern","Brass Leaf","Warm Pins","Soft Chimes","Cloud Steps","Slow Pebble","Paper Tines"},1);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("PresetPrev","Previous Sound",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("PresetNext","Next Sound",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("MotionMode","Motion Mode",{"Tremolo","Auto Pan"},1);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Sync","Motion Sync",{"Off","On"},1);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Shape","Motion Shape",{"Sine","Triangle","Square","S&H"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("DelayDivision","Delay Division",{"1/1","1/1 D","1/1 T","1/2","1/2 D","1/2 T","1/4","1/4 D","1/4 T","1/8","1/8 D","1/8 T","1/16","1/16 D","1/16 T","1/32","1/32 D","1/32 T"},10);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("DelayFeedback","Delay Feedback",0,72,28,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("RoomTime","Room Time",0,100,55,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("RoomDamp","Room Damping",0,100,50,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Hammer","Hammer",0,100,35,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Velocity","Velocity Sens",0,100,65,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Resonance","Resonance",0,100,32,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Noise","Mechanical",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Pick","Pick Position",0,100,22,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Detune","String Detune",0,12,1.5,"ct",1);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Tuning","Tuning",430,450,440,"Hz",1);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Pedal","Sustain Pedal",{"Off","On"},1);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("PitchRange","Bend Range",1,12,2,"st",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("VelocityCurve","Velocity Curve",{"Soft","Linear","Firm"},1);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("KeyOff","Key Off",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("MotionDelay","Motion Delay",0,3000,0,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("MotionFade","Motion Fade",0,3000,0,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("DriftRate","Drift Rate",0.05,3,0.24,"Hz",2);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("ChorusRate","Chorus Rate",0.05,2,0.31,"Hz",2);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("ChorusDepth","Chorus Depth",0,100,50,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Roll","Roll Dice",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Undo","Undo",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("UnlockAll","Unlock All",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Panic","All Notes Off",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("InputMode","Keyboard / Pads",{"Notes","Chords","Split"},2);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Key","Key",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Scale","Reference Scale",{"Major","Minor"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Transpose","Transpose",-12,12,0,"st",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Strum","Strum",0,160,18,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("StrumDirection","Strum Direction",{"Up","Down","Alternate"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("StrumVelocity","Strum Velocity",0,30,5,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("ChordGlide","Chord Glide",{"Off","On"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::floatParam("Glide","Glide Time",0,1200,90,"ms",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Slots","Chord Slots",{"4","8","12","16"},0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("Page","Slot Page",{"1\u20134","5\u20138","9\u201312","13\u201316"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("PagePrev","Previous Slots",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("PageNext","Next Slots",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("SlotsAdd","Add Four Slots",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("SlotsRemove","Remove Four Slots",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Smooth","Smooth Top Notes",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordPrev","Previous Progression",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordNext","Next Progression",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordPreset1","Progression 1",{"Sunday Soul","Amber Changes","Side Street","Soft Landing","Blue Hour","Quiet Turn","Warm Window","Copper Circle","Dusty Steps","Night Bloom","Little Bridge","Soft Motion","Low Lantern","Porch Chords","Last Train","Velvet Loop"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordPreset2","Progression 2",{"Sunday Soul","Amber Changes","Side Street","Soft Landing","Blue Hour","Quiet Turn","Warm Window","Copper Circle","Dusty Steps","Night Bloom","Little Bridge","Soft Motion","Low Lantern","Porch Chords","Last Train","Velvet Loop"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordPreset3","Progression 3",{"Sunday Soul","Amber Changes","Side Street","Soft Landing","Blue Hour","Quiet Turn","Warm Window","Copper Circle","Dusty Steps","Night Bloom","Little Bridge","Soft Motion","Low Lantern","Porch Chords","Last Train","Velvet Loop"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordPreset4","Progression 4",{"Sunday Soul","Amber Changes","Side Street","Soft Landing","Blue Hour","Quiet Turn","Warm Window","Copper Circle","Dusty Steps","Night Bloom","Little Bridge","Soft Motion","Low Lantern","Porch Chords","Last Train","Velvet Loop"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root1","Root 1",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type1","Chord Type 1",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing1","Voicing 1",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave1","Octave 1",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote1","Pad Note 1",-1,127,24,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root2","Root 2",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},7);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type2","Chord Type 2",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},6);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing2","Voicing 2",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave2","Octave 2",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote2","Pad Note 2",-1,127,26,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root3","Root 3",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type3","Chord Type 3",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},3);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing3","Voicing 3",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave3","Octave 3",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote3","Pad Note 3",-1,127,28,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root4","Root 4",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},9);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type4","Chord Type 4",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing4","Voicing 4",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave4","Octave 4",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote4","Pad Note 4",-1,127,29,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root5","Root 5",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type5","Chord Type 5",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing5","Voicing 5",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave5","Octave 5",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote5","Pad Note 5",-1,127,31,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root6","Root 6",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},7);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type6","Chord Type 6",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},6);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing6","Voicing 6",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave6","Octave 6",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote6","Pad Note 6",-1,127,33,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root7","Root 7",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type7","Chord Type 7",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},3);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing7","Voicing 7",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave7","Octave 7",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote7","Pad Note 7",-1,127,35,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root8","Root 8",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},9);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type8","Chord Type 8",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing8","Voicing 8",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave8","Octave 8",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote8","Pad Note 8",-1,127,36,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root9","Root 9",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type9","Chord Type 9",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing9","Voicing 9",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave9","Octave 9",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote9","Pad Note 9",-1,127,38,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root10","Root 10",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},7);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type10","Chord Type 10",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},6);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing10","Voicing 10",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave10","Octave 10",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote10","Pad Note 10",-1,127,40,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root11","Root 11",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type11","Chord Type 11",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},3);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing11","Voicing 11",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave11","Octave 11",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote11","Pad Note 11",-1,127,41,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root12","Root 12",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},9);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type12","Chord Type 12",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing12","Voicing 12",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave12","Octave 12",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote12","Pad Note 12",-1,127,43,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root13","Root 13",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type13","Chord Type 13",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing13","Voicing 13",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave13","Octave 13",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote13","Pad Note 13",-1,127,45,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root14","Root 14",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},7);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type14","Chord Type 14",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},6);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing14","Voicing 14",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave14","Octave 14",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote14","Pad Note 14",-1,127,47,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root15","Root 15",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type15","Chord Type 15",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},3);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing15","Voicing 15",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave15","Octave 15",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote15","Pad Note 15",-1,127,48,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Root16","Root 16",{"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"},9);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Type16","Chord Type 16",{"m7","maj7","m9","maj9","7","9","13","sus2","m","Major","7b9","m6","6","m6/9","6/9","m11","m13","maj9#11","maj13","7sus4","9sus4","13sus4","7#9","7b13","7alt","m7b5","dim7","7#5","mMaj7","add9","madd9","sus4"},2);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Voicing16","Voicing 16",{"Root","First","Open","Second","Third","Fourth"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Octave16","Octave 16",-2,2,0,"oct",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PadNote16","Pad Note 16",-1,127,50,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Hold1","Touch Chord 1",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Learn1","Learn Pad 1",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("ChordName1","Chord Name 1",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit0","Chord Edit 1",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit1","Chord Edit 2",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit2","Chord Edit 3",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit3","Chord Edit 4",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Hold2","Touch Chord 2",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Learn2","Learn Pad 2",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("ChordName2","Chord Name 2",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit4","Chord Edit 5",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit5","Chord Edit 6",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit6","Chord Edit 7",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit7","Chord Edit 8",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Hold3","Touch Chord 3",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Learn3","Learn Pad 3",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("ChordName3","Chord Name 3",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit8","Chord Edit 9",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit9","Chord Edit 10",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit10","Chord Edit 11",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit11","Chord Edit 12",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Hold4","Touch Chord 4",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("Learn4","Learn Pad 4",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("ChordName4","Chord Name 4",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit12","Chord Edit 13",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit13","Chord Edit 14",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit14","Chord Edit 15",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Edit15","Chord Edit 16",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Character3Label","Character3Label",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Character4Label","Character4Label",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("CurrentChordPreset","CurrentChordPreset",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("PageInfo","PageInfo",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("LearnInfo","LearnInfo",0,1,0,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockTone","Lock Tone",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockBody","Lock Body",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockCharacter3","Lock Character3",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockCharacter4","Lock Character4",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockAttack","Lock Attack",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDecay","Lock Decay",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockSustain","Lock Sustain",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockRelease","Lock Release",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockRate","Lock Rate",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDepth","Lock Depth",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDrift","Lock Drift",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockWidth","Lock Width",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDrive","Lock Drive",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockChorus","Lock Chorus",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDelay","Lock Delay",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockRoom","Lock Room",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockModel","Lock Model",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockMotionMode","Lock MotionMode",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockSync","Lock Sync",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockShape","Lock Shape",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDelayDivision","Lock DelayDivision",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDelayFeedback","Lock DelayFeedback",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockRoomTime","Lock RoomTime",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockRoomDamp","Lock RoomDamp",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockHammer","Lock Hammer",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockVelocity","Lock Velocity",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockResonance","Lock Resonance",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockNoise","Lock Noise",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockPick","Lock Pick",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDetune","Lock Detune",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockVelocityCurve","Lock VelocityCurve",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockKeyOff","Lock KeyOff",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockMotionDelay","Lock MotionDelay",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockMotionFade","Lock MotionFade",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockDriftRate","Lock DriftRate",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockChorusRate","Lock ChorusRate",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("LockChorusDepth","Lock ChorusDepth",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("ChordEditing","Chord Editor",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ0","Main Screen 1",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ1","Main Screen 2",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ2","Main Screen 3",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ3","Main Screen 4",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ4","Main Screen 5",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ5","Main Screen 6",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ6","Main Screen 7",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ7","Main Screen 8",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ8","Main Screen 9",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ9","Main Screen 10",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ10","Main Screen 11",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ11","Main Screen 12",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ12","Main Screen 13",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ13","Main Screen 14",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ14","Main Screen 15",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("MainQ15","Main Screen 16",0,1,0,"",4);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("Hiss","Noise",0,100,0,"%",0);v.automatable=true;p.addParam(v);}
{auto v=mpc::choiceParam("LockHiss","Lock Texture Noise",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("CommandPads","Pad Commands",{"Off","On"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("CmdPrevNote","Previous Preset Pad",-1,127,25,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("CmdNextNote","Next Preset Pad",-1,127,26,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("CmdDiceNote","Dice Pad",-1,127,24,"",0);v.automatable=false;p.addParam(v);}
{auto v=mpc::choiceParam("CommandLearn","Learn Action",{"Off","PRESET PREV","PRESET NEXT","ROLL DICE"},0);v.automatable=false;p.addParam(v);}
{auto v=mpc::floatParam("CommandInfo","Pad Actions",0,1,0,"",0);v.automatable=false;p.addParam(v);}
}
static constexpr int kMainKeys[]={kTone,kBody,kCharacter3,kHiss,kAttack,kDecay,kSustain,kRelease,kRate,kDepth,kDrift,kWidth,kDrive,kChorus,kDelay,kRoom};
static constexpr int kSoundKeys[]={kTone,kBody,kCharacter3,kCharacter4,kAttack,kDecay,kSustain,kRelease,kRate,kDepth,kDrift,kWidth,kDrive,kChorus,kDelay,kRoom,kModel,kMotionMode,kSync,kShape,kDelayDivision,kDelayFeedback,kRoomTime,kRoomDamp,kHammer,kVelocity,kResonance,kNoise,kPick,kDetune,kVelocityCurve,kKeyOff,kMotionDelay,kMotionFade,kDriftRate,kChorusRate,kChorusDepth,kHiss};
static constexpr int kSoundLocks[]={kLockTone,kLockBody,kLockCharacter3,kLockCharacter4,kLockAttack,kLockDecay,kLockSustain,kLockRelease,kLockRate,kLockDepth,kLockDrift,kLockWidth,kLockDrive,kLockChorus,kLockDelay,kLockRoom,kLockModel,kLockMotionMode,kLockSync,kLockShape,kLockDelayDivision,kLockDelayFeedback,kLockRoomTime,kLockRoomDamp,kLockHammer,kLockVelocity,kLockResonance,kLockNoise,kLockPick,kLockDetune,kLockVelocityCurve,kLockKeyOff,kLockMotionDelay,kLockMotionFade,kLockDriftRate,kLockChorusRate,kLockChorusDepth,kLockHiss};
