// Generated parameter declarations. Stable index order from parameters.json.
enum Key { kMaster, kQ1, kQ2, kQ3, kQ4, kQ5, kQ6, kQ7, kQ8, kQ9, kQ10, kQ11, kQ12, kQ13, kQ14, kQ15, kQ16, kSwing, kSnareLate, kHatShift, kRndNudge, kDensity, kGhost, kVelSpread, kRandomAmt, kFillSlot, kFillDensity, kFillEvery, kVariation, kDrive, kDust, kRoom, kDuck, kKit, kPattern, kKitLock, kPatternLock, kScope, kRoll, kUndo, kKeep, kTrigger, kRecord, kCancel, kSaveKit, kLoadKit, kPreview, kFollow, kPreviewBpm, kEditor, kFillLength, kRecMode, kCountIn, kNoteSelect, kNoteNudge, kNoteVelocity, kNoteGate, kNoteChance, kNoteDelete, kStatus, kPosition, kRecordedCount, kPad0, kLevel0, kTune0, kDecay0, kPan0, kSample0, kChoke0, kRoute0, kPadNote0, kPad1, kLevel1, kTune1, kDecay1, kPan1, kSample1, kChoke1, kRoute1, kPadNote1, kPad2, kLevel2, kTune2, kDecay2, kPan2, kSample2, kChoke2, kRoute2, kPadNote2, kPad3, kLevel3, kTune3, kDecay3, kPan3, kSample3, kChoke3, kRoute3, kPadNote3, kPad4, kLevel4, kTune4, kDecay4, kPan4, kSample4, kChoke4, kRoute4, kPadNote4, kPad5, kLevel5, kTune5, kDecay5, kPan5, kSample5, kChoke5, kRoute5, kPadNote5, kPad6, kLevel6, kTune6, kDecay6, kPan6, kSample6, kChoke6, kRoute6, kPadNote6, kPad7, kLevel7, kTune7, kDecay7, kPan7, kSample7, kChoke7, kRoute7, kPadNote7, kChokeTime, kSeparate, kLowCut, kModRate, kModDepth, kUserKit, kEditDrum, kGated, kGridSelect, kGrid0, kGrid1, kGrid2, kGrid3, kGrid4, kGrid5, kGrid6, kGrid7, kGrid8, kGrid9, kGrid10, kGrid11, kGrid12, kGrid13, kGrid14, kGrid15, kGrid16, kGrid17, kGrid18, kGrid19, kGrid20, kGrid21, kGrid22, kGrid23, kGrid24, kGrid25, kGrid26, kGrid27, kGrid28, kGrid29, kGrid30, kGrid31, kGrid32, kGrid33, kGrid34, kGrid35, kGrid36, kGrid37, kGrid38, kGrid39, kGrid40, kGrid41, kGrid42, kGrid43, kGrid44, kGrid45, kGrid46, kGrid47, kGrid48, kGrid49, kGrid50, kGrid51, kGrid52, kGrid53, kGrid54, kGrid55, kGrid56, kGrid57, kGrid58, kGrid59, kGrid60, kGrid61, kGrid62, kGrid63, kBankInfo, kGridBar, kGridTap, kKitPrev, kKitNext, kPatternPrev, kPatternNext, kLibrarySample0, kLibrarySample1, kLibrarySample2, kLibrarySample3, kLibrarySample4, kLibrarySample5, kLibrarySample6, kLibrarySample7, kLibraryBeat, kGrooveTemplate, kLibraryOpen, kLibraryKind, kLibraryRole, kLibraryPage, kLibraryPrev, kLibraryNext, kLibraryLegacy, kLibraryBeatOff, kGrooveOff, kLibraryPick0, kLibraryPick1, kLibraryPick2, kLibraryPick3, kLibraryPick4, kLibraryPick5, kLibraryPick6, kLibraryPick7, kLibraryRow0, kLibraryRow1, kLibraryRow2, kLibraryRow3, kLibraryRow4, kLibraryRow5, kLibraryRow6, kLibraryRow7, kLibraryInfo, kLibraryFolder, kLibraryBack, kLibraryCurrentSound, kLibraryPlaybackInfo, kFactoryKit, kKitDiceMode, kKitBrowserPage, kKitPagePrev, kKitPageNext, kDiceInfo, kFactoryReload, kSoundKick, kSoundSnare, kSoundHat, kSoundPerc, kSoundBrowserGroup, kSoundBrowserPage, kSoundOpen, kSoundPrev, kSoundNext, kSoundPick0, kSoundPick1, kSoundPick2, kSoundPick3, kSoundPick4, kSoundPick5, kSoundPick6, kSoundPick7, kSoundRow0, kSoundRow1, kSoundRow2, kSoundRow3, kSoundRow4, kSoundRow5, kSoundRow6, kSoundRow7, kSoundInfo, kRuleKick, kRuleSnare, kRuleHat, kRulePerc, kActionInfo, kHatGrid, kPhraseState, kHatRhythm, kHatPhraseState, kHatRhythmPage, kXKit, kXSoundKick, kXSoundSnare, kXSoundHat, kXSoundPerc, kXSample0, kXSample1, kXSample2, kXSample3, kXSample4, kXSample5, kXSample6, kXSample7, kXBeat, kXGroove, kXKitPage, kXSoundPage, kXLibraryPage, kXLibraryFolder, kPackFilter, kPackPrev, kPackNext, kKitOpen, kPackInfo, kKitRow0, kKitRow1, kKitRow2, kKitRow3, kKitRow4, kKitRow5, kKitRow6, kKitRow7, kKitPick0, kKitPick1, kKitPick2, kKitPick3, kKitPick4, kKitPick5, kKitPick6, kKitPick7, kXQKit, kXQKick, kXQSnare, kXQHat, kXQPerc, kParamCount };
inline void declareParameters(mpc::Plugin& plugin) {
{ auto p=mpc::floatParam("Master","Master",0,1,0.7,"",2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q1","Screen 1",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q2","Screen 2",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q3","Screen 3",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q4","Screen 4",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q5","Screen 5",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q6","Screen 6",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q7","Screen 7",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q8","Screen 8",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q9","Screen 9",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q10","Screen 10",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q11","Screen 11",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q12","Screen 12",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q13","Screen 13",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q14","Screen 14",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q15","Screen 15",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Q16","Screen 16",0,1,0,"",3); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Swing","Swing",50,75,54,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("SnareLate","Snare Late",-40,60,12,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("HatShift","Hat Shift",-40,40,-4,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("RndNudge","Rnd Nudge",0,30,4,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Density","Density",0,100,100,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Ghost","Ghost",0,100,15,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("VelSpread","Vel Spread",0,50,8,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("RandomAmt","Random Amt",0,100,35,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("FillSlot","Fill Select",{"A","B","C","D"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("FillDensity","Fill Density",0,100,100,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("FillEvery","Fill Every",{"Off","4 Bars","8 Bars","16 Bars"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Variation","Variation",0,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Drive","Drive",0,100,10,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Dust","Dust",0,100,10,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Room","Room",0,100,8,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Duck","Duck",0,100,15,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Kit","Kit",{"Dusty Velvet","Basement Soul","Amber Room"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Pattern","Pattern",{"Velvet Stoop","Basement Bounce","Sunday Steps"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitLock","Kit Lock",{"Off","On"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("PatternLock","Pattern Lock",{"Off","On"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Scope","Scope",{"Kit Only","Kit + Pattern","Kit + Groove","Kit + Fill","All","Kit + FX"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Roll","Roll Dice",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Undo","Undo",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Keep","Keep",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Trigger","Trigger Fill",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Record","Rec Fill",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Cancel","Cancel",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SaveKit","Save Kit",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LoadKit","Load Kit",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Preview","Preview",{"Off","On"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Follow","Follow MPC",{"Off","On"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PreviewBpm","Preview BPM",40,180,92,"BPM",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Editor","Fill Editor",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("FillLength","Fill Length",{"1 Beat","2 Beats","1 Bar","2 Bars","4 Bars"},2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("RecMode","Rec Mode",{"Replace","Overdub"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("CountIn","Count-in",{"Off","1 Bar","2 Bars"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("NoteSelect","Note Select",0,127,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("NoteNudge","Note Nudge",-60,60,0,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("NoteVelocity","Note Velocity",1,127,100,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("NoteGate","Note Gate",5,500,80,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("NoteChance","Note Chance",0,100,100,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("NoteDelete","Delete Note",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Status","Status",0,6,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Position","Position",0,9999,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("RecordedCount","Fill Notes",0,128,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad0","Pad 1",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level0","Level 1",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune0","Tune 1",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay0","Decay 1",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan0","Pan 1",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample0","Sample 1",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke0","Choke 1",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route0","Output 1",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote0","Pad Note 1",0,127,24,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad1","Pad 2",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level1","Level 2",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune1","Tune 2",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay1","Decay 2",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan1","Pan 2",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample1","Sample 2",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke1","Choke 2",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route1","Output 2",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote1","Pad Note 2",0,127,26,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad2","Pad 3",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level2","Level 3",-48,6,-6,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune2","Tune 3",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay2","Decay 3",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan2","Pan 3",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample2","Sample 3",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke2","Choke 3",0,16,1,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route2","Output 3",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote2","Pad Note 3",0,127,28,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad3","Pad 4",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level3","Level 4",-48,6,-6,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune3","Tune 4",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay3","Decay 4",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan3","Pan 4",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample3","Sample 4",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke3","Choke 4",0,16,1,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route3","Output 4",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},3); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote3","Pad Note 4",0,127,29,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad4","Pad 5",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level4","Level 5",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune4","Tune 5",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay4","Decay 5",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan4","Pan 5",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample4","Sample 5",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke4","Choke 5",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route4","Output 5",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},4); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote4","Pad Note 5",0,127,31,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad5","Pad 6",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level5","Level 6",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune5","Tune 6",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay5","Decay 6",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan5","Pan 6",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample5","Sample 6",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke5","Choke 6",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route5","Output 6",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},5); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote5","Pad Note 6",0,127,33,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad6","Pad 7",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level6","Level 7",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune6","Tune 7",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay6","Decay 7",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan6","Pan 7",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample6","Sample 7",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke6","Choke 7",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route6","Output 7",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},6); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote6","Pad Note 7",0,127,35,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pad7","Pad 8",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Level7","Level 8",-48,6,0,"dB",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Tune7","Tune 8",-12,12,0,"st",1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Decay7","Decay 8",20,2000,500,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Pan7","Pan 8",-100,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Sample7","Sample 8",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("Choke7","Choke 8",0,16,0,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Route7","Output 8",{"Main/Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},7); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PadNote7","Pad Note 8",0,127,36,"",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("ChokeTime","Choke Time",1,30,5,"ms",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Separate","Separate Out",{"Off","On"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("LowCut","Low Cut",0,300,0,"Hz",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("ModRate","Mod Rate",0.05,8,0.3,"Hz",2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("ModDepth","Mod Depth",0,100,0,"%",0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("UserKit","User Kit",{"1","2","3","4","5","6","7","8"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("EditDrum","Edit Drum",{"Kick","Snare","Closed Hat"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("Gated","Gated Samples",{"Off","On"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("GridSelect","Grid Select",0,64,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid0","Cell 1",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid1","Cell 2",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid2","Cell 3",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid3","Cell 4",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid4","Cell 5",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid5","Cell 6",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid6","Cell 7",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid7","Cell 8",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid8","Cell 9",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid9","Cell 10",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid10","Cell 11",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid11","Cell 12",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid12","Cell 13",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid13","Cell 14",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid14","Cell 15",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid15","Cell 16",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid16","Cell 17",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid17","Cell 18",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid18","Cell 19",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid19","Cell 20",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid20","Cell 21",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid21","Cell 22",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid22","Cell 23",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid23","Cell 24",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid24","Cell 25",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid25","Cell 26",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid26","Cell 27",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid27","Cell 28",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid28","Cell 29",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid29","Cell 30",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid30","Cell 31",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid31","Cell 32",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid32","Cell 33",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid33","Cell 34",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid34","Cell 35",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid35","Cell 36",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid36","Cell 37",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid37","Cell 38",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid38","Cell 39",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid39","Cell 40",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid40","Cell 41",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid41","Cell 42",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid42","Cell 43",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid43","Cell 44",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid44","Cell 45",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid45","Cell 46",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid46","Cell 47",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid47","Cell 48",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid48","Cell 49",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid49","Cell 50",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid50","Cell 51",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid51","Cell 52",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid52","Cell 53",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid53","Cell 54",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid54","Cell 55",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid55","Cell 56",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid56","Cell 57",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid57","Cell 58",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid58","Cell 59",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid59","Cell 60",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid60","Cell 61",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid61","Cell 62",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid62","Cell 63",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("Grid63","Cell 64",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("BankInfo","Sound Bank",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("GridBar","View Bar",{"1","2","3","4"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("GridTap","Grid Tap",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPrev","KitPrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitNext","KitNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("PatternPrev","PatternPrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("PatternNext","PatternNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample0","Library Sound 1",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample1","Library Sound 2",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample2","Library Sound 3",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample3","Library Sound 4",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample4","Library Sound 5",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample5","Library Sound 6",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample6","Library Sound 7",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibrarySample7","Library Sound 8",0,11039,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryBeat","Library Beat",0,2,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("GrooveTemplate","Groove Template",0,93,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryOpen","Library Browser",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryKind","Library Type",{"Kits","Chops","Loops","Beats","Grooves"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryRole","Destination",{"Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryPage","Library Page",0,2000,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPrev","LibraryPrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryNext","LibraryNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryLegacy","LibraryLegacy",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryBeatOff","LibraryBeatOff",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("GrooveOff","GrooveOff",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick0","Library Pick 1",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick1","Library Pick 2",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick2","Library Pick 3",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick3","Library Pick 4",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick4","Library Pick 5",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick5","Library Pick 6",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick6","Library Pick 7",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryPick7","Library Pick 8",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow0","Library Row 1",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow1","Library Row 2",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow2","Library Row 3",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow3","Library Row 4",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow4","Library Row 5",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow5","Library Row 6",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow6","Library Row 7",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryRow7","Library Row 8",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryInfo","Library Info",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryFolder","Library Folder",0,2000,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("LibraryBack","Library Back",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryCurrentSound","Current Sound",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("LibraryPlaybackInfo","Library Playback",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("FactoryKit","Drum Kit",{"Custom / Legacy","V01 / Velvet01","V01 / Velvet02","V01 / Velvet03","V01 / Velvet04","V01 / Velvet05","V01 / Velvet06","V01 / Velvet07","V01 / Velvet08","V01 / Velvet09","V01 / Velvet10","V01 / Velvet11","V01 / Velvet12","V01 / Velvet13","V01 / Velvet14","V01 / Velvet15","V01 / Velvet16","V02 / Pocket01","V02 / Pocket02","V02 / Pocket03","V02 / Pocket04","V02 / Pocket05","V02 / Pocket06","V02 / Pocket07","V02 / Pocket08","V02 / Pocket09","V02 / Pocket10","V02 / Pocket11","V02 / Pocket12","V02 / Pocket13","V02 / Pocket14","V02 / Pocket15","V02 / Pocket16","V03 / Amber01","V03 / Amber02","V03 / Amber03","V03 / Amber04","V03 / Amber05","V03 / Amber06","V03 / Amber07","V03 / Amber08","V03 / Amber09","V03 / Amber10","V03 / Amber11","V03 / Amber12","V03 / Amber13","V03 / Amber14","V03 / Amber15","V03 / Amber16","V04 / Loose01","V04 / Loose02","V04 / Loose03","V04 / Loose04","V04 / Loose05","V04 / Loose06","V04 / Loose07","V04 / Loose08","V04 / Loose09","V04 / Loose10","V04 / Loose11","V04 / Loose12","V04 / Loose13","V04 / Loose14","V04 / Loose15","V04 / Loose16","V05 / Honey01","V05 / Honey02","V05 / Honey03","V05 / Honey04","V05 / Honey05","V05 / Honey06","V05 / Honey07","V05 / Honey08","V05 / Honey09","V05 / Honey10","V05 / Honey11","V05 / Honey12","V05 / Honey13","V05 / Honey14","V05 / Honey15","V05 / Honey16","V06 / Dust01","V06 / Dust02","V06 / Dust03","V06 / Dust04","V06 / Dust05","V06 / Dust06","V06 / Dust07","V06 / Dust08","V06 / Dust09","V06 / Dust10","V06 / Dust11","V06 / Dust12","V06 / Dust13","V06 / Dust14","V06 / Dust15","V06 / Dust16"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitDiceMode","Dice Mode",{"Whole Kit","Mix Parts"},0); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitBrowserPage","Kit Page",0,12,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPagePrev","KitPagePrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPageNext","KitPageNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("DiceInfo","Dice Result",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("FactoryReload","Reload Kit",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundKick","Kick",0,440,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundSnare","Snare",0,546,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundHat","Hat",0,376,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundPerc","Perc Set",0,244,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundBrowserGroup","Sound Role",{"Kick","Snare","Hat Pairs","Perc Sets"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundBrowserPage","Sound Page",0,100,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundOpen","Role Sounds",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPrev","SoundPrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundNext","SoundNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick0","SoundPick0",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick1","SoundPick1",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick2","SoundPick2",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick3","SoundPick3",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick4","SoundPick4",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick5","SoundPick5",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick6","SoundPick6",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("SoundPick7","SoundPick7",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow0","Sound Row 1",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow1","Sound Row 2",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow2","Sound Row 3",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow3","Sound Row 4",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow4","Sound Row 5",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow5","Sound Row 6",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow6","Sound Row 7",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundRow7","Sound Row 8",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("SoundInfo","Sound Info",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("RuleKick","Kick Rule",{"LOCK","POCKET","LOOSE","BUSY"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("RuleSnare","Snare Rule",{"LOCK","POCKET","LOOSE","BUSY"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("RuleHat","Hat Rule",{"LOCK","POCKET","LOOSE","BUSY"},2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::choiceParam("RulePerc","Perc Rule",{"LOCK","POCKET","LOOSE","BUSY"},2); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("ActionInfo","Last Action",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("HatGrid","Hat Grid",{"1/4","1/8","1/16","1/32 Roll"},1); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("PhraseState","Phrase State",0,262143,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("HatRhythm","Hat Rhythm",{"1/4","1/4 + FILL","1/4 + ROLL","1/8","1/8 + FILL","1/8 + ROLL","1/16","1/16 + FILL","1/16 + ROLL","1/8 DOT","1/8 DOT + FILL","1/8 DOT + ROLL","1/16 DOT","1/16 DOT + FILL","1/16 DOT + ROLL"},3); p.automatable=true; plugin.addParam(p); }
{ auto p=mpc::floatParam("HatPhraseState","Hat Phrase State",0,4095,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("HatRhythmPage","Hat Rhythm Page",{"1 / 2","2 / 2"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XKit","Expansion Kit",0,65535,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSoundKick","Kick Catalog",0,65535,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSoundSnare","Snare Catalog",0,65535,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSoundHat","Hat Catalog",0,65535,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSoundPerc","Perc Catalog",0,65535,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample0","Expansion Sound 1",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample1","Expansion Sound 2",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample2","Expansion Sound 3",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample3","Expansion Sound 4",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample4","Expansion Sound 5",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample5","Expansion Sound 6",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample6","Expansion Sound 7",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSample7","Expansion Sound 8",0,1048575,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XBeat","Expansion Beat",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XGroove","Expansion Groove",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XKitPage","Kit Catalog Page",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XSoundPage","Sound Catalog Page",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XLibraryPage","Library Catalog Page",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XLibraryFolder","Library Catalog Folder",0,131071,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("PackFilter","Expansion Filter",0,128,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("PackPrev","PackPrev",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("PackNext","PackNext",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitOpen","KitOpen",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("PackInfo","Expansion Info",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow0","Kit Row 1",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow1","Kit Row 2",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow2","Kit Row 3",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow3","Kit Row 4",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow4","Kit Row 5",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow5","Kit Row 6",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow6","Kit Row 7",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("KitRow7","Kit Row 8",0,1,0,"",0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick0","Kit Pick 1",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick1","Kit Pick 2",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick2","Kit Pick 3",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick3","Kit Pick 4",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick4","Kit Pick 5",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick5","Kit Pick 6",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick6","Kit Pick 7",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::choiceParam("KitPick7","Kit Pick 8",{"Off","On"},0); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XQKit","Kit Select",0,1,0,"",4); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XQKick","Kick Select",0,1,0,"",4); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XQSnare","Snare Select",0,1,0,"",4); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XQHat","Hat Select",0,1,0,"",4); p.automatable=false; plugin.addParam(p); }
{ auto p=mpc::floatParam("XQPerc","Perc Select",0,1,0,"",4); p.automatable=false; plugin.addParam(p); }
}
