import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import java.time.LocalDate as LocalDate
import java.time.format.DateTimeFormatter as DateTimeFormatter
import com.github.javafaker.Faker as Faker
import com.flick.katalon.api.HelperValidate as HelperValidate

Faker faker = new Faker(new Locale('in-ID'))

WS.sendRequestAndVerify(findTestObject('null', [('SilviStaging') : GlobalVariable.SilviStaging
            , ('id') : 'cf51ae33-cf66-4725-8acc-46dad7f7eafa', ('merchant_phone_number') : '0827123456789', ('merchant_email') : 'namdosan@mailnesia.com'
            , ('merchant_name') : 'Samsan Restoran', ('merchant_address') : 'Jalan Harapan No.3, Lebak Bulus, Cilandak, Jakarta Selatan, DKI Jakarta'
            , ('merchant_activation_type') : 0]))

