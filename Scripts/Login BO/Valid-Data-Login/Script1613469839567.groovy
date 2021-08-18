import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

AuthToken = WS.sendRequest(findTestObject('Backoffice Login/Login BO',[('BaseFlickStaging') : GlobalVariable.BaseFlickStaging]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(AuthToken.getResponseBodyContent())

def value_token = result.data['access-token']

GlobalVariable.BOBearerToken = value_token


