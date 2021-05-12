package tech.skagedal.tracker

object ProcessEnvironment {
    val DEBUG = System.getenv("DEBUG") == "true"
}