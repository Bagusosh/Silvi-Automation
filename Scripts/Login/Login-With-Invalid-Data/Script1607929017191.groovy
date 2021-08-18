import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.github.javafaker.Faker as Faker
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

Faker faker = new Faker(new Locale('in-ID'))

String randomNumber = faker.number()

String phoneNumber = faker.phoneNumber()

AuthToken = WS.sendRequest(findTestObject(null))

