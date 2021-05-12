package tech.skagedal.tracker.commands

import com.github.ajalt.clikt.core.CliktCommand
import tech.skagedal.tracker.tracker.TimeTracker
import java.time.LocalDate
import java.time.LocalTime

class TrackStopCommand(
    val timeTracker: TimeTracker
) : CliktCommand(name = "stop") {
    override fun run() {
        val date = LocalDate.now()
        val time = LocalTime.now()
        timeTracker.stopTracking(date, time)
        println("Stopped tracking")
    }
}