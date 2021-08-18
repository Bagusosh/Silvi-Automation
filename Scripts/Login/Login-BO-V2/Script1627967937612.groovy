import static com.kms.katalon.core.testdata.TestDataFactory.findTestData 
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

BearerToken=WS.sendRequest((RequestObject)findTestObject('SilviV2/Auth/Admin/Login', [('Silvi-V2-Staging') : GlobalVariable.SilviV2Staging]))

def slurper = new JsonSlurper()

def result = slurper.parseText(BearerToken.getResponseBodyContent())

def value_token = result.data['token']

GlobalVariable.adminToken = value_token

print value_token
