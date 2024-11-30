ThisBuild / version := "0.1"
ThisBuild / scalaVersion := "2.12.18"
ThisBuild / organization := "com.craigjb"

val spinalVersion = "dev"

// val spinalCore = "com.github.spinalhdl" %% "spinalhdl-core" % spinalVersion
// val spinalLib = "com.github.spinalhdl" %% "spinalhdl-lib" % spinalVersion
// val spinalIdslPlugin = compilerPlugin(
//   "com.github.spinalhdl" %% "spinalhdl-idsl-plugin" % spinalVersion
// )

lazy val spinalCore = ProjectRef(file("../SpinalHDL"), "core")
lazy val spinalLib = ProjectRef(file("../SpinalHDL"), "lib")
lazy val spinalIdslPlugin = ProjectRef(file("../SpinalHDL"), "idslplugin")
lazy val vexRiscv = ProjectRef(file("../VexRiscv"), "root")

lazy val root = (project in file("."))
  .settings(
    Compile / scalaSource := baseDirectory.value / "hw" / "spinal",
    scalacOptions += s"-Xplugin:${new File(baseDirectory.value + s"/../SpinalHDL/idslplugin/target/scala-2.12/spinalhdl-idsl-plugin_2.12-$spinalVersion.jar")}",
    scalacOptions += s"-Xplugin-require:idsl-plugin"
  )
  .dependsOn(spinalCore, spinalLib, spinalIdslPlugin, vexRiscv)

fork := true
