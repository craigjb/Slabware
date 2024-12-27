// package slabware
//
// import spinal.core._
// import spinal.core.sim._
//
// object TestSlabGrid extends App {
//   val spinalConfig = SpinalConfig(
//     defaultClockDomainFrequency = FixedFrequency(50 MHz)
//   )
//
//   SimConfig
//     .withConfig(spinalConfig)
//     .withWave
//     .compile(
//       new SlabGrid(
//         numSpiClusters = 2,
//         waitTime = 100 ns
//       )
//     )
//     .doSim { dut =>
//       dut.clockDomain.forkStimulus(10)
//       dut.clockDomain.waitSampling(500000)
//     }
// }
