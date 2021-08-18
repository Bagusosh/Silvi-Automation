import static com.kms.katalon.core.testdata.TestDataFactory.findTestData 
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

AuthToken = WS.sendRequest((RequestObject)findTestObject('Backoffice Login/Login BO', [('BaseFlickStaging') : GlobalVariable.BOProd]))

def slurper = new JsonSlurper()

def result = slurper.parseText(AuthToken.getResponseBodyContent())

def value_token = result.data['access-token']

GlobalVariable.BOBearerToken = value_token

def categoryMap = [:]

for (int rowNum = 1; rowNum <= findTestData('Asmara/Asmara_Category').getRowNumbers(); rowNum++) {
    categoryProduct = findTestData('Asmara/Asmara_Category').getValue(1, rowNum)
	
	if (categoryProduct == ""){ 
		break
	}
	
    jsonCategory = WS.sendRequest((RequestObject)findTestObject('POS Backoffice/Merchant Product Management/Create Category', [('BOBaseSilvi') : GlobalVariable.BOSilviProd
                , ('merchantId') : GlobalVariable.merchantId, ('categoryName') : categoryProduct, ('bearerToken') : GlobalVariable.BOBearerToken]))
    def categoryResponse = slurper.parseText(jsonCategory.getResponseBodyContent())
    def categoryName = categoryResponse.data['category_name']
	def categoryId = categoryResponse.data['id']

    categoryMap.put(categoryName, categoryId)
	print categoryMap
	
	
}


for (int rowNum = 2; rowNum <= findTestData('Asmara/Asmara').getRowNumbers(); rowNum++) {
    productCategory = findTestData('Asmara/Asmara').getValue(1, rowNum)
	productName = findTestData('Asmara/Asmara').getValue(2, rowNum)
    productSellingPrice = findTestData('Asmara/Asmara').getValue(3, rowNum)
	productPurchasePrice = findTestData('Asmara/Asmara').getValue(4, rowNum)
	productStock = findTestData('Asmara/Asmara').getValue(5, rowNum)
	productMinStock = findTestData('Asmara/Asmara').getValue(6, rowNum)
	productDescription = findTestData('Asmara/Asmara').getValue(7, rowNum)
    String productCategoryId = categoryMap.get(productCategory)
	
	if ( productName == ""){
		break
	}
	
	WS.delay(1)

	WS.sendRequest((RequestObject)findTestObject('POS Backoffice/Merchant Product Management/Create Product', [('BOBaseSilvi') : GlobalVariable.BOSilviProd
                , ('merchantId') : GlobalVariable.merchantId, ('bearerToken') : GlobalVariable.BOBearerToken, ('productName') : productName
                , ('productCategoryId') : productCategoryId, ('productSellingPrice') : productSellingPrice, ('productPurchasePrice') : getProductPurchasePrice(
                    productPurchasePrice), ('productStock') : productStock, ('productMinStock') : getProductMinStock(productMinStock)
                , ('productDescription') : getProductDescription(productDescription)]))
	
}
 
def getProductPurchasePrice(def purchasePrice) {
    if (purchasePrice == 'empty') {
        purchasePrice = 0
    }
    return purchasePrice
}

def getProductMinStock(def productMinStock) {
    if (productMinStock == 'empty') {
        productMinStock = 10
    }
    
    return productMinStock
}

def getProductDescription(def productDescription) {
    if (productDescription == 'empty') {
        productDescription = ''
    }
    
    return productDescription
}

