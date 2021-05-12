package tech.skagedal.tracker.tracker

import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import java.nio.file.FileSystems
import java.time.LocalDate

internal class TrackerRepositoryTest {
    private val restoredHome = System.getProperty("user.home")

    @BeforeEach
    internal fun setUp() {
        System.setProperty("user.home", "/home/the-user")
    }

    @AfterEach
    internal fun tearDown() {
        System.setProperty("user.home", restoredHome)
    }

    @Test
    internal fun `test file name formatting`() {
        val repository = createRepository()

        val p = repository.pathForWeekTrackerFile(LocalDate.of(2020, 7, 11))
        assertEquals("/home/the-user/.simons-assistant/data/tracker/2020-W28.txt", p.toString())
    }

    @Test
    internal fun `week numbers are padded with zeros`() {
        val repository = createRepository()

        val p = repository.pathForWeekTrackerFile(LocalDate.of(2020, 1, 4))
        assertEquals("/home/the-user/.simons-assistant/data/tracker/2020-W01.txt", p.toString())
    }

    @Test
    internal fun `week-based year is used`() {
        val repository = createRepository()

        val p = repository.pathForWeekTrackerFile(LocalDate.of(2019, 12, 30))
        assertEquals("/home/the-user/.simons-assistant/data/tracker/2020-W01.txt", p.toString())
    }

    @Test
    internal fun `create a default document`() {
        val repository = createRepository()
        val document = repository.defaultDocument(LocalDate.of(2020, 7, 11))
        fun dayWithBlank(year: Int, month: Int, day: Int) = Day(
            LocalDate.of(year, month, day),
            listOf(Line.Blank)
        )
        assertEquals(
            Document(
                emptyList(),
                listOf(
                    dayWithBlank(2020, 7, 6),
                    dayWithBlank(2020, 7, 7),
                    dayWithBlank(2020, 7, 8),
                    dayWithBlank(2020, 7, 9),
                    dayWithBlank(2020, 7, 10)
                )
            ),
            document
        )
    }

    private fun createRepository() = TrackerRepository(FileSystems.getDefault(), Serializer())
}