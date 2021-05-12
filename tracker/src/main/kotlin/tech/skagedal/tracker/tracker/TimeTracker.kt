package tech.skagedal.tracker.tracker

import java.nio.file.Files
import java.time.Duration
import java.time.LocalDate
import java.time.LocalTime
import java.time.temporal.ChronoUnit

class TrackerFileAlreadyHasOpenShiftException(override val message: String?) : RuntimeException(message)
class TrackerFileHasNoOpenShiftOnThatDayException(override val message: String?) : RuntimeException(message)
class TrackerFileHasMultipleOpenShiftsInOneDayException(override val message: String?) : RuntimeException(message)

class TimeTracker(
    private val trackerRepository: TrackerRepository,
    private val serializer: Serializer,
    private val standardWorkDayMinutes: Long = 60 * 8
) {
    fun weekReportForDate(date: LocalDate): WeekReport {
        val path = trackerRepository.weekTrackerFileCreateIfNeeded(date)
        val document = Files.newBufferedReader(path).use { serializer.parseDocument(it) }
        val currentTime = LocalTime.now()

        return weekReportForDateInDocument(document, date, currentTime)
    }

    fun weekReportForDateInDocument(
        document: Document,
        date: LocalDate,
        currentTime: LocalTime
    ): WeekReport {
        return WeekReport(
            document.days.find { it.date == date }?.let { trackedDurationForDay(it, currentTime) } ?: Duration.ZERO,
            minutesForDocument(document, currentTime),
            document.hasOpenShift()
        )
    }

    fun minutesForDocument(document: Document, currentTime: LocalTime) =
        document.days.map { trackedDurationForDay(it, currentTime) }.sum()

    fun trackedDurationForDay(day: Day, currentTime: LocalTime) =
        day.lines.map { trackedDurationForLine(it, currentTime) }.sum()

    fun trackedDurationForLine(line: Line, currentTime: LocalTime) =
        when (line) {
            is Line.Comment -> Duration.ZERO
            is Line.DayHeader -> Duration.ZERO
            is Line.OpenShift -> Duration.between(line.startTime, currentTime)
            is Line.ClosedShift -> Duration.between(line.startTime, line.stopTime)
            is Line.SpecialDay -> Duration.of(standardWorkDayMinutes, ChronoUnit.MINUTES)
            is Line.SpecialShift -> Duration.between(line.startTime, line.stopTime)
            Line.Blank -> Duration.ZERO
        }

    fun startTracking(date: LocalDate, time: LocalTime) {
        val path = trackerRepository.weekTrackerFileCreateIfNeeded(date)
        val document = Files.newBufferedReader(path).use { serializer.parseDocument(it) }
        val newDocument = documentWithTrackingStarted(document, date, time)
        Files.newBufferedWriter(path).use { serializer.writeDocument(newDocument, it) }
    }

    fun stopTracking(date: LocalDate, time: LocalTime) {
        val path = trackerRepository.pathForWeekTrackerFile(date)
        val document = Files.newBufferedReader(path).use { serializer.parseDocument(it) }
        val newDocument = documentWithTrackingStopped(document, date, time)
        Files.newBufferedWriter(path).use { serializer.writeDocument(newDocument, it) }
    }

    fun documentWithTrackingStarted(document: Document, date: LocalDate, time: LocalTime): Document {
        if (document.days.any { it.lines.any { it is Line.OpenShift } }) {
            throw TrackerFileAlreadyHasOpenShiftException("There is already an open shift")
        }

        val day = document.days.find { it.date == date }
        return if (day != null) {
            document.copy(
                days = document.days.map {
                    if (it === day) {
                        it.copy(
                            lines = it.lines.addingShift(Line.OpenShift(time))
                        )
                    } else {
                        it
                    }
                }
            )
        } else {
            val daysBefore = document.days.takeWhile { it.date.isBefore(date) }.let { days ->
                if (days.isNotEmpty()) {
                    days.dropLast(1) + listOf(days.last().addingBlankLine())
                } else {
                    days
                }
            }
            val daysAfter = document.days.dropWhile { it.date.isBefore(date) }
            val insertedDays = listOf(Day(date, listOf(Line.OpenShift(time))))
            document.copy(
                days = daysBefore + insertedDays + daysAfter
            )
        }
    }

    fun documentWithTrackingStopped(document: Document, date: LocalDate, time: LocalTime): Document {
        val day = document.days.find { it.date == date }
            ?: throw TrackerFileHasNoOpenShiftOnThatDayException("No open shift on $date")
        return document.copy(
            days = document.days.map {
                if (it === day) {
                    it.copy(
                        lines = it.lines.closingShift(time)
                    )
                } else {
                    it
                }
            }
        )
    }

    private fun List<Line>.addingShift(line: Line): List<Line> {
        val before = dropLastWhile { ! it.isShift() }
        val after = takeLastWhile { ! it.isShift() }
        return before + listOf(line) + after
    }

    private fun List<Line>.closingShift(stopTime: LocalTime): List<Line> {
        val openShiftCount = filter { it is Line.OpenShift }.size
        return when (openShiftCount) {
            1 -> map {
                if (it is Line.OpenShift)
                    Line.ClosedShift(it.startTime, stopTime)
                else
                    it
            }
            0 -> throw TrackerFileHasNoOpenShiftOnThatDayException("No open shift on that day")
            else -> throw TrackerFileHasMultipleOpenShiftsInOneDayException("More than one open shift in that day")
        }
    }

    private fun Document.hasOpenShift() = days.any { it.hasOpenShift() }
    private fun Day.hasOpenShift() = lines.any { it is Line.OpenShift }

    private fun Day.addingBlankLine() = copy(
        lines = lines + listOf(Line.Blank)
    )

    private fun Iterable<Duration>.sum() = fold(Duration.ZERO, Duration::plus)
}

