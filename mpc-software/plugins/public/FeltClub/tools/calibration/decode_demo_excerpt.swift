import Foundation
import AVFoundation
func decode() throws {
let args=CommandLine.arguments
if args.count != 5 { fatalError("input output start duration") }
let input=try AVAudioFile(forReading:URL(fileURLWithPath:args[1]))
let rate=input.processingFormat.sampleRate
let start=Double(args[3])!,duration=Double(args[4])!
input.framePosition=AVAudioFramePosition(start*rate)
let settings:[String:Any]=[AVFormatIDKey:kAudioFormatLinearPCM,AVSampleRateKey:rate,AVNumberOfChannelsKey:1,AVLinearPCMBitDepthKey:16,AVLinearPCMIsFloatKey:false,AVLinearPCMIsBigEndianKey:false]
let output=try AVAudioFile(forWriting:URL(fileURLWithPath:args[2]),settings:settings)
let monoFormat=AVAudioFormat(standardFormatWithSampleRate:rate,channels:1)!
let source=AVAudioPCMBuffer(pcmFormat:input.processingFormat,frameCapacity:4096)!
let mono=AVAudioPCMBuffer(pcmFormat:monoFormat,frameCapacity:4096)!
var remaining=AVAudioFramePosition(duration*rate),written:AVAudioFramePosition=0
while remaining>0 {
 try input.read(into:source,frameCount:AVAudioFrameCount(min(remaining,4096)))
 let frames=source.frameLength;if frames==0 {break};mono.frameLength=frames
 for i in 0..<Int(frames){var sum:Float=0;for ch in 0..<Int(source.format.channelCount){sum+=source.floatChannelData![ch][i]};mono.floatChannelData![0][i]=sum/Float(source.format.channelCount)}
 try output.write(from:mono);written+=AVAudioFramePosition(frames);remaining-=AVAudioFramePosition(frames)
}
print("Decoded",written,"mono frames",rate,"Hz")

}
try decode()
