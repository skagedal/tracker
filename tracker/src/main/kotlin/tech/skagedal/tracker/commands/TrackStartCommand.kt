package tech.skagedal.tracker.commands

import com.github.ajalt.clikt.core.CliktCommand
import tech.skagedal.tracker.tracker.TimeTracker
import java.time.LocalDate
import java.time.LocalTime

class TrackStartCommand(
    val timeTracker: TimeTracker
) : CliktCommand(name = "start") {
    override fun run() {
        val date = LocalDate.now()
        val time = LocalTime.now()
        timeTracker.startTracking(date, time)
        println("Started tracking")
    }
}
