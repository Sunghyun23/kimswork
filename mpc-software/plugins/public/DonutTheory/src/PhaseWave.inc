// Per-instance lock-free reflection ownership: concurrent UI writes remain live.
std::array<std::atomic<std::uintptr_t>,8> waveAliasThreads_{};
bool waveAliasReflection()const{const auto token=donutThreadToken();for(const auto&t:waveAliasThreads_)if(t.load(std::memory_order_acquire)==token)return true;return false;}
void syncWaveAliases(){
 if(getInt(kWave1)==donut::legacyWave(getInt(kWave1Full))&&getInt(kWave2)==donut::legacyWave(getInt(kWave2Full)))return;
 const auto token=donutThreadToken();for(auto&t:waveAliasThreads_){std::uintptr_t empty=0;if(t.compare_exchange_strong(empty,token,std::memory_order_acq_rel)){
  setFromEngine(kWave1,float(donut::legacyWave(getInt(kWave1Full))));setFromEngine(kWave2,float(donut::legacyWave(getInt(kWave2Full))));t.store(0,std::memory_order_release);return;
 }} // A later block reconciles if all bounded reflection slots are occupied.
}
void seedExpandedWaves(){const int a=getInt(kWave1),b=getInt(kWave2);setFromEngine(kWave1Full,float(a));setFromEngine(kWave2Full,float(b));syncWaveAliases();}
static float phaseCycles(float degrees){if(std::abs(degrees-82.8f)<.0001f)return .23f;return std::fmod(degrees/360.f,1.f);}
