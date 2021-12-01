import org.scalatest.funsuite.AnyFunSuite

class MainTest extends AnyFunSuite {
  val filename = "example.txt"
  val parsed = Main.readFile(filename)

  test("Main.sweeper") {
    assert(Main.sweeper(parsed) == 7)
  }
  test("Main.slidingWindow") {
    assert(Main.sweeping_window(parsed) == 5)
  }
}
