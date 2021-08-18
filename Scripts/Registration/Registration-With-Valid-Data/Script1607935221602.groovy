import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.github.javafaker.Faker as Faker
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

Faker faker = new Faker(new Locale('in-ID'))

String firstPhoneNumber = faker.phoneNumber().subscriberNumber()

String secondPhoneNumber = faker.phoneNumber().extension()

String name = faker.name().name()

String fullName = faker.name().fullName()

String email = faker.internet().emailAddress()

String PIN = faker.number().digits(6)

String fakeAddress = faker.address().fullAddress()

Regist = WS.sendRequestAndVerify(findTestObject('Flick API Core/Auth/OTP Request', [('BaseFlickStaging') : GlobalVariable.BaseFlickStaging
            , ('tipe') : 'registrasi', ('hp') : 0+8+firstPhoneNumber+secondPhoneNumber]))

println(0+firstPhoneNumber+secondPhoneNumber)

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(Regist.getResponseBodyContent())

def token_type = result.data.tipe_token

def token = result.data.token

registration = WS.sendRequest(findTestObject('Flick API Core/Account-Registration/Registration', [('BaseFlickStaging') : GlobalVariable.BaseFlickStaging
            , ('hp') : 0+8+firstPhoneNumber+secondPhoneNumber, ('name') : name, ('fullName') : fullName, ('email') : email, ('PIN') : PIN, ('token_type') : 'merchant'
            , ('merchant_type') : 'offline', ('business_type') : 'makanan', ('address') : fakeAddress, ('token') : token]))



