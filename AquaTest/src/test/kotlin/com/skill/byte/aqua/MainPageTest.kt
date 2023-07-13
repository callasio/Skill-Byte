package com.skill.byte.aqua

import com.codeborne.selenide.Condition.attribute
import com.codeborne.selenide.Condition.visible
import com.codeborne.selenide.Configuration
import com.codeborne.selenide.Selectors.*
import com.codeborne.selenide.Selenide
import com.codeborne.selenide.Selenide.element
import com.codeborne.selenide.Selenide.open
import org.openqa.selenium.chrome.ChromeOptions
import com.codeborne.selenide.logevents.SelenideLogger
import io.qameta.allure.selenide.AllureSelenide
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.*

class MainPageTest {
    private val mainPage = MainPage()

    companion object {
        @JvmStatic
        @BeforeAll
        fun setUpAll() {
            Configuration.browserSize = "1280x800"
            SelenideLogger.addListener("allure", AllureSelenide())
        }
    }

    @BeforeEach
    fun setUp() {
        Configuration.browserCapabilities =
            ChromeOptions().addArguments(
                "--remote-allow-origins=*",
                "--headless"
            )
        open("http://localhost:1420/")
    }

    @Test
    fun search() {
        assertEquals(mainPage.pClickTheTauriVite.text(), "Click on the Tauri, Vite, and React logos to learn more.")
    }
}
