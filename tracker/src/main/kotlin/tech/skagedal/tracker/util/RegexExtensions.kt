package tech.skagedal.tracker.util

import java.util.regex.Matcher
import java.util.regex.Pattern

fun Pattern.matcherIfMatches(string: String): Matcher? =
    matcher(string).let { if (it.matches()) it else null }
