package tech.skagedal.tracker.util

fun <T> ((T) -> Boolean).not() = { t: T -> !this(t) }

