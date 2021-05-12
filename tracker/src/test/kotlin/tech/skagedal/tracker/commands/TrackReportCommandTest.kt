package tech.skagedal.tracker.commands

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class TrackReportCommandTest {
    @Test
    internal fun `test formatting`() {
        val intHour: Int = 1
        val longHour: Long = 1
        assertEquals("1 hour", intHour.withUnit("hour", "hours"))
        assertEquals("1 hour", longHour.withUnit("hour", "hours"))

        val intHours: Int = 2
        val longHours: Int = 2
        assertEquals("2 hours", intHours.withUnit("hour", "hours"))
        assertEquals("2 hours", longHours.withUnit("hour", "hours"))
    }
}