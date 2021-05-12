package tech.skagedal.tracker.tracker

import tech.skagedal.tracker.util.matcherIfMatches
import tech.skagedal.tracker.util.segment
import tech.skagedal.tracker.util.splitSublists
import java.io.Reader
import java.io.Writer
import java.time.LocalDate
import java.time.LocalTime
import java.time.format.DateTimeFormatter
import java.time.format.DateTimeFormatterBuilder
import java.time.format.TextStyle
import java.time.temporal.ChronoField
import java.time.temporal.ChronoUnit
import java.util.Locale
import java.util.regex.Pattern

class Serializer {
    private val formatter = DateTimeFormatterBuilder()
        .appendValue(ChronoField.HOUR_OF_DAY, 2)
        .appendLiteral(':')
        .appendValue(ChronoField.MINUTE_OF_HOUR, 2)
        .toFormatter()

    fun writeDocument(document: Document, writer: Writer) {
        writeLines(document.preamble, writer)
        for (day in document.days) {
            writer.write("[${headerDateFormat(day.date)}]\n")
            writeLines(day.lines, writer)
        }
    }

    private fun writeLines(lines: List<Line>, writer: Writer) {
        for (line in lines) {
            when (line) {
                is Line.Comment -> writer.write("# ${line.text}\n")
                is Line.DayHeader -> writer.write("[${headerDateFormat(line.date)}]\n")
                is Line.OpenShift -> writer.write("* ${formatTime(line.startTime)}-\n")
                is Line.ClosedShift -> writer.write("* ${formatTime(line.startTime)}-${formatTime(line.stopTime)}\n")
                is Line.SpecialDay -> writer.write("* ${line.text}\n")
                is Line.SpecialShift -> writer.write("* ${line.text} ${formatTime(line.startTime)}-${formatTime(line.stopTime)}\n")
                Line.Blank -> writer.write("\n")
            }
        }
    }

    private fun headerDateFormat(date: LocalDate): String {
        val weekDay = date.dayOfWeek.getDisplayName(TextStyle.FULL_STANDALONE, Locale.ENGLISH).toLowerCase()
        val isoDate = date.format(DateTimeFormatter.ISO_LOCAL_DATE)
        return "$weekDay $isoDate"
    }

    private fun formatTime(time: LocalTime) =
        time.truncatedTo(ChronoUnit.MINUTES).format(formatter)

    // Parser

    private val commentPattern = Pattern.compile("^# (?<text>.*)$")
    private val dayHeaderPattern = Pattern.compile("^\\[[a-z]+\\s+(?<year>[0-9]{4})-(?<month>[0-9]{2})-(?<day>[0-9]{2})]$")
    private val openShiftPattern = Pattern.compile("^\\* (?<hour>[0-9]{2}):(?<minute>[0-9]{2})-$")
    private val closedShiftPattern =
        Pattern.compile("^\\* (?<startHour>[0-9]{2}):(?<startMinute>[0-9]{2})-(?<stopHour>[0-9]{2}):(?<stopMinute>[0-9]{2})$")
    private val specialShiftPattern =
        Pattern.compile("^\\* (?<text>[A-Za-z]+) (?<startHour>[0-9]{2}):(?<startMinute>[0-9]{2})-(?<stopHour>[0-9]{2}):(?<stopMinute>[0-9]{2})$")
    private val specialDayPattern = Pattern.compile("^\\* (?<text>[A-Za-z]+)$")
    private val blankPattern = Pattern.compile("^\\s*$")

    fun parseDocument(reader: Reader): Document {
        val scannedLines = reader.readLines().mapIndexed(this::parseLine)
        val (preamble, dayLines) = scannedLines.segment { it !is Line.DayHeader }
        val days = dayLines.splitSublists { it is Line.DayHeader }.map { lines ->
            val dayHeader = lines.first() as Line.DayHeader
            val contentLines = lines.drop(1)
            Day(dayHeader.date, contentLines)
        }
        return Document(
            preamble,
            days
        )
    }

    private fun parseLine(lineNumber: Int, string: String): Line =
        parseComment(string)
            ?: parseDayHeader(string)
            ?: parseOpenShift(string)
            ?: parseClosedShift(string)
            ?: parseSpecialShift(string)
            ?: parseSpecialDay(string)
            ?: parseBlank(string)
            ?: throw ParseException(lineNumber + 1, string)

    class ParseException(lineNumber: Int, string: String) : RuntimeException("Couldn't parse line $lineNumber:\n$string")

    private fun parseComment(string: String): Line? =
        commentPattern.matcherIfMatches(string)?.let {
            Line.Comment(it.group("text"))
        }

    private fun parseDayHeader(string: String): Line? =
        dayHeaderPattern.matcherIfMatches(string)?.let {
            Line.DayHeader(LocalDate.of(it.group("year").toInt(), it.group("month").toInt(), it.group("day").toInt()))
        }

    private fun parseOpenShift(string: String): Line? =
        openShiftPattern.matcherIfMatches(string)?.let {
            Line.OpenShift(LocalTime.of(it.group("hour").toInt(), it.group("minute").toInt()))
        }

    private fun parseClosedShift(string: String): Line? =
        closedShiftPattern.matcherIfMatches(string)?.let {
            Line.ClosedShift(
                LocalTime.of(it.group("startHour").toInt(), it.group("startMinute").toInt()),
                LocalTime.of(it.group("stopHour").toInt(), it.group("stopMinute").toInt())
            )
        }

    private fun parseSpecialShift(string: String): Line? =
        specialShiftPattern.matcherIfMatches(string)?.let {
            Line.SpecialShift(
                it.group("text"),
                LocalTime.of(it.group("startHour").toInt(), it.group("startMinute").toInt()),
                LocalTime.of(it.group("stopHour").toInt(), it.group("stopMinute").toInt())
            )
        }

    private fun parseSpecialDay(string: String): Line? =
        specialDayPattern.matcherIfMatches(string)?.let {
            Line.SpecialDay(it.group("text"))
        }

    private fun parseBlank(string: String): Line? =
        blankPattern.matcherIfMatches(string)?.let { Line.Blank }
}