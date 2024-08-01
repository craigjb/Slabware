ThisBuild / version := "0.1"
ThisBuild / scalaVersion := "2.11.12"
ThisBuild / organization := "com.craigjb"

val spinalVersion = "1.10.1"
val spinalCore = "com.github.spinalhdl" %% "spinalhdl-core" % spinalVersion
val spinalLib = "com.github.spinalhdl" %% "spinalhdl-lib" % spinalVersion
val spinalIdslPlugin = compilerPlugin(
  "com.github.spinalhdl" %% "spinalhdl-idsl-plugin" % spinalVersion
)

lazy val vexRiscv = RootProject(
  uri("https://github.com/SpinalHDL/VexRiscv.git")
)

lazy val root = (project in file("."))
  .settings(
    Compile / scalaSource := baseDirectory.value / "hw" / "spinal",
    libraryDependencies ++= Seq(spinalCore, spinalLib, spinalIdslPlugin)
  )
  .dependsOn(vexRiscv)

fork := true
