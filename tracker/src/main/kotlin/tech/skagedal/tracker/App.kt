package tech.skagedal.tracker

import org.slf4j.LoggerFactory
import tech.skagedal.tracker.commands.TrackEditCommand
import tech.skagedal.tracker.commands.TrackReportCommand
import tech.skagedal.tracker.commands.TrackStartCommand
import tech.skagedal.tracker.commands.TrackStopCommand
import tech.skagedal.tracker.commands.TrackerCommand
import tech.skagedal.tracker.tracker.Serializer
import tech.skagedal.tracker.tracker.TimeTracker
import tech.skagedal.tracker.tracker.TrackerRepository
import tech.skagedal.tracker.util.ProcessRunner
import java.nio.file.FileSystems

private object Main {
    val logger = LoggerFactory.getLogger(javaClass)

    fun main(args: Array<String>) {
        logger.info("Starting simons-assistant")

        val tracker = buildTracker()
        tracker.main(args)
    }

    private fun buildTracker(): TrackerCommand {
        val serializer = Serializer()
        val fileSystem = FileSystems.getDefault()
        val repository = TrackerRepository(
            fileSystem,
            serializer
        )
        val timeTracker = TimeTracker(
            repository,
            serializer
        )
        val processRunner = ProcessRunner()

        val tracker = TrackerCommand(
            listOf(
                TrackStartCommand(timeTracker),
                TrackStopCommand(timeTracker),
                TrackEditCommand(repository, processRunner),
                TrackReportCommand(timeTracker)
            )
        )
        return tracker
    }
}

fun main(args: Array<String>) = Main.main(args)
