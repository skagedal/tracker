package tech.skagedal.tracker.util

/**
 * Splits a list, given a predicate, into both the "takeWhile" and the "dropWhile" parts.
 * It's named "span" after the operation in Scala.
 * This could be optimized into just traversing the list once.  The predicate must have no side-effects.
 */
inline fun <T> List<T>.segment(predicate: (T) -> Boolean): Pair<List<T>, List<T>> =
    Pair(takeWhile(predicate), dropWhile(predicate))

/**
 * Splits a list into sublists, each starting with an element matching [pivotPredicate].
 * Assumes that the list, if non-empty, starts with an element matching the predicate.
 */
fun <T> List<T>.splitSublists(pivotPredicate: (T) -> Boolean): List<List<T>> =
    if (isNotEmpty()) {
        val pivotElement = first()
        assert(pivotPredicate(pivotElement))
        val (nonPivotElements, rest) = drop(1).segment(pivotPredicate.not())
        listOf(listOf(pivotElement) + nonPivotElements) + rest.splitSublists(pivotPredicate)
    } else {
        emptyList()
    }

fun <T> List<T>.split(predicate: (T) -> Boolean) =
    segment(predicate.not()).let { (preamble, list) ->
        listOf(preamble) + list.splitSublists(predicate)
    }
