import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

BearerToken = WS.sendRequest(((findTestObject('SilviV2/Auth/Admin/Login', [('SilviV2Staging') : GlobalVariable.SilviV2Staging])) as RequestObject))

def slurper = new JsonSlurper()

def result = slurper.parseText(BearerToken.getResponseBodyContent())

def value_token = result.data['token']

GlobalVariable.adminToken = value_token

print(value_token)

def categoryMap = [:]

for (int rowNum = 1; rowNum <= findTestData('Asmara/Asmara_Category').getRowNumbers(); rowNum++) {
    categoryProduct = findTestData('Asmara/Asmara_Category').getValue(1, rowNum)

    if (categoryProduct == '') {
        break
    }
    
    jsonCategory = WS.sendRequest(((findTestObject('SilviV2/Product Category/Create Product Category', [('SilviV2Staging') : GlobalVariable.SilviV2Staging
                    , ('merchantId') : GlobalVariable.merchantId, ('categoryName') : categoryProduct])) as RequestObject))

    def categoryResponse = slurper.parseText(jsonCategory.getResponseBodyContent())

    def categoryName = categoryResponse.data.product_category['product_category_name']

    def categoryId = categoryResponse.data.product_category['product_category_id']

    categoryMap.put(categoryName, categoryId)

    print(categoryMap)
}

for (int rowNum = 2; rowNum <= findTestData('Asmara/Asmara').getRowNumbers(); rowNum++) {
	productCategory = findTestData('Asmara/Asmara').getValue(1, rowNum)
	productName = findTestData('Asmara/Asmara').getValue(2, rowNum)
	productSellingPrice = findTestData('Asmara/Asmara').getValue(3, rowNum)
	productPurchasePrice = findTestData('Asmara/Asmara').getValue(4, rowNum)
	activeStatus = findTestData('Asmara/Asmara').getValue(5, rowNum)
	productDescription = findTestData('Asmara/Asmara').getValue(6, rowNum)
	String productCategoryId = categoryMap.get(productCategory)
	
	if ( productName == ""){
		break
	}
	
	WS.delay(1)
	
	WS.sendRequest((RequestObject)findTestObject('SilviV2/Product/Create Product', [('SilviV2Staging') : GlobalVariable.SilviV2Staging, ('productDescription') : getProductDescription(productDescription)
				, ('productName') : productName, ('productCategoryId') : productCategoryId, ('activeStatus') : getactiveStatus(activeStatus), ('productPurchasePrice') : getProductPurchasePrice(
					productPurchasePrice), ('productSellingPrice') : productSellingPrice, ('bearerToken') : GlobalVariable.adminToken, ('merchantId') : GlobalVariable.merchantId]))

	
}
 
def getProductPurchasePrice(def purchasePrice) {
	if (purchasePrice == 'empty') {
		purchasePrice = 0
	}
	return purchasePrice
}

def getProductDescription(def productDescription) {
	if (productDescription == 'empty') {
		productDescription = ''
	}
	
	return productDescription
}

def getactiveStatus(def activeStatus) {
	if (activeStatus == 'empty') {
		activeStatus = 'true'
	}
	
	return activeStatus
}
