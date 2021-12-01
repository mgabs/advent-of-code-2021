import scala.io.Source
import scala.language.postfixOps

object Main extends App {

  def readFile(filename: String): Seq[Int] = {
    val bufferedSource = io.Source.fromFile(filename)
    val lines =
      (for (line <- bufferedSource.getLines()) yield line.toInt).toList
    bufferedSource.close
    lines
  }

  def sweeper(x: Seq[Int]): Int = {
    var count = -1
    var prev = 0

    for (i <- x) {
      if (i > prev) {
        count += 1
      }
      prev = i
    }
    count
  }

  def sweeping_window(x: Seq[Int]): Int = {
    var count = -1
    var prev_val = 0

    var windows = x.sliding(3, 1)
    // println(windows)
    for (window <- windows) {
      val win_sum = window.sum
      if (win_sum > prev_val) {
        count += 1

      }
      prev_val = win_sum
    }
    count
  }

  val filename = "input.txt"

  val parsed = readFile(filename)

  val sweeped = sweeper(parsed)
  val windowed = sweeping_window(parsed)

  println(s"Number of Increases: $sweeped")
  println(s"Number of Increase windows: $windowed")
}
