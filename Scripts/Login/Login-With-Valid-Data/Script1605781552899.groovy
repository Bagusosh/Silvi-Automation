import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

AuthToken = WS.sendRequest(((findTestObject('Flick API Core/Auth/Login', [('BaseFlickStaging') : GlobalVariable.BaseFlickStaging
                , ('PIN') : '150600', ('hp') : '087741331517'])) as RequestObject))

println(AuthToken)

def slurper = new JsonSlurper()

def result = slurper.parseText(AuthToken.getResponseBodyContent())

println(result)

def value_token = result.data['access-token']

def value_user_id = result.data.user.id

GlobalVariable.BearerToken = value_token

GlobalVariable.userId = value_user_id

println(GlobalVariable.userId)


