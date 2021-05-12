package tech.skagedal.tracker.tracker

import tech.skagedal.tracker.assistantDataDirectory
import java.nio.file.FileAlreadyExistsException
import java.nio.file.FileSystem
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardOpenOption
import java.time.LocalDate
import java.time.LocalTime
import java.time.format.DateTimeFormatter
import java.time.temporal.WeekFields

sealed class Line {
    data class Comment(val text: String) : Line()
    data class DayHeader(val date: LocalDate) : Line()
    data class OpenShift(val startTime: LocalTime) : Line()
    data class ClosedShift(val startTime: LocalTime, val stopTime: LocalTime) : Line()
    data class SpecialDay(val text: String) : Line()
    data class SpecialShift(val text: String, val startTime: LocalTime, val stopTime: LocalTime) : Line()
    object Blank : Line()
}

fun Line.isShift() = when (this) {
    is Line.OpenShift, is Line.ClosedShift, is Line.SpecialShift -> true
    is Line.Comment, is Line.DayHeader, is Line.SpecialDay, is Line.Blank -> false
}

data class Day(
    val date: LocalDate,
    val lines: List<Line>
)

data class Document(
    val preamble: List<Line>,
    val days: List<Day>
)

class TrackerRepository(
    val fileSystem: FileSystem,
    val serializer: Serializer
) {
    private val formatter = DateTimeFormatter.ofPattern("Y-'W'ww")

    fun weekTrackerFileCreateIfNeeded(date: LocalDate): Path {
        val path = pathForWeekTrackerFile(date)
        Files.createDirectories(path.parent)
        try {
            Files.newBufferedWriter(path, StandardOpenOption.CREATE_NEW).use { writer ->
                serializer.writeDocument(defaultDocument(date), writer)
            }
        } catch (e: FileAlreadyExistsException) {
            // Ignored, expected
        }
        return path
    }

    fun pathForWeekTrackerFile(date: LocalDate) =
        fileSystem.assistantDataDirectory().resolve("tracker").resolve(date.format(formatter) + ".txt")

    fun defaultDocument(date: LocalDate): Document {
        val days = (1..5).map { dayNumber ->
            Day(
                date.with(WeekFields.ISO.dayOfWeek(), dayNumber.toLong()),
                listOf(
                    Line.Blank
                )
            )
        }
        return Document(emptyList(), days)
    }
}