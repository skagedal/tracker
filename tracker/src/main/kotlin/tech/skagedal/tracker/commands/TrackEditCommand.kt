package tech.skagedal.tracker.commands

import com.github.ajalt.clikt.core.CliktCommand
import tech.skagedal.tracker.tracker.TrackerRepository
import tech.skagedal.tracker.util.ProcessRunner
import java.time.LocalDate

class TrackEditCommand(
    val trackerRepository: TrackerRepository,
    val processRunner: ProcessRunner
) : CliktCommand(name = "edit") {
    override fun run() {
        val path = trackerRepository.weekTrackerFileCreateIfNeeded(LocalDate.now())
        processRunner.runEditor(path)
    }
}