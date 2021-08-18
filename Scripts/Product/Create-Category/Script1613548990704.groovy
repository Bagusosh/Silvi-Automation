import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import internal.GlobalVariable as GlobalVariable

AuthToken = WS.sendRequest(findTestObject('Backoffice Login/Login BO',[('BaseFlickStaging') : GlobalVariable.BaseFlickStaging]))

def slurper = new groovy.json.JsonSlurper()
def result = slurper.parseText(AuthToken.getResponseBodyContent())
def value_token = result.data['access-token']
GlobalVariable.BOBearerToken = value_token

def categoryMap = [:]

for (int rowNum = 1; rowNum <= findTestData('null').getRowNumbers(); rowNum++) {
    categoryProduct = findTestData('null').getValue(1, rowNum)
    jsonCategory = WS.sendRequest((RequestObject)findTestObject('POS Backoffice/Merchant Product Management/Create Category', [('BO-Silvi-Staging') : GlobalVariable.BOSilviStaging
                , ('merchantId') : GlobalVariable.merchantId, ('categoryName') : categoryProduct, ('bearerToken') : GlobalVariable.BOBearerToken]))
	def categoryResponse = slurper.parseText(jsonCategory.getResponseBodyContent())
	
	def categoryName = categoryResponse.data['category_name']
	def categoryId = categoryResponse.data['id']
	
	categoryMap.put(categoryName,categoryId)
}

for (int rowNum = 1; rowNum <= findTestData('null').getRowNumbers(); rowNum++) {
	productCategory = findTestData('null').getValue(1, rowNum)
	productName = findTestData('null').getValue(2, rowNum)
	productSellingPrice = findTestData('DcoffeeNyok').getValue(3, rowNum)
	productPurchasePrice =  findTestData('DcoffeeNyok').getValue(4, rowNum)
	productCapitalPrice =  findTestData('DcoffeeNyok').getValue(5, rowNum)
	productStock =  findTestData('DcoffeeNyok').getValue(6, rowNum)
	productMinStock =  findTestData('DcoffeeNyok').getValue(7, rowNum)
	productDescription =  findTestData('DcoffeeNyok').getValue(8, rowNum)
	
	if (productPurchasePrice == "") {
		productPurhcasePrice = "test"
	print(productPurchasePrice)
	String productCategoryId = categoryMap.get(productCategory)
	print(productCategoryId)
			product = WS.sendRequest((RequestObject)findTestObject('POS Backoffice/Merchant Product Management/Create Product', [('BO-Silvi-Staging') : GlobalVariable.BOSilviStaging
			, ('merchantId') : GlobalVariable.merchantId, ('bearerToken') : GlobalVariable.BOBearerToken, ('productName') : productName
			, ('productCategoryId') : productCategoryId, ('productSellingPrice') : productSellingPrice, ('productPurchasePrice') : productPurchasePrice, ('productStock') : productStock
			, ('productMinStock') : productMinStock, ('productDescription') : productDescription]))
			

			}
}




