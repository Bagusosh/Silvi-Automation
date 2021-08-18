import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.github.javafaker.Faker as Faker
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

Faker faker = new Faker(new Locale('in-ID'))

String randomNumber = faker.number()
faker.phoneNumber.subscriberNumber()

AuthToken = WS.sendRequestAndVerify(findTestObject('Flick API Core/Auth/Login - Invalid', [('BaseFlickStaging') : GlobalVariable.BaseFlickStaging
            , ('PIN') : randomNumber, ('hp') : '082277831094']))

def slurper = new JsonSlurper()

def result = slurper.parseText(AuthToken.getResponseBodyContent())
print(result)

