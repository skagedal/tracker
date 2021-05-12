package tech.skagedal.tracker.util

class ProcessExitedUnsuccessfullyException(val command: List<String>, val code: Int) : RuntimeException(
    "The command ${command} exited with status code ${code}."
)