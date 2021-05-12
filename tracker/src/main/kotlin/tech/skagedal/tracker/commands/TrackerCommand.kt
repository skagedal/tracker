package tech.skagedal.tracker.commands

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.core.subcommands

class TrackerCommand(
    private val subcommands: List<CliktCommand>
) : CliktCommand(
    name = "tracker"
) {
    init {
        subcommands(subcommands)
    }

    override fun run() = Unit
}