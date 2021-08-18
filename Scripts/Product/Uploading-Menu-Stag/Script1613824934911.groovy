import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

AuthToken = WS.sendRequest(findTestObject('SilviV2/Auth/Login', [('Silvi-V2-Staging') : (GlobalVariable.Silvi - V2) - Staging]))

def slurper = new JsonSlurper()

def result = slurper.parseText(AuthToken.getResponseBodyContent())

def value_token = result.data['access-token']

GlobalVariable.BOBearerToken = value_token

def categoryMap = [:]

for (int rowNum = 1; rowNum <= findTestData('Meneer_Category').getRowNumbers(); rowNum++) {
    categoryProduct = findTestData('Meneer_Category').getValue(1, rowNum)

    jsonCategory = WS.sendRequest(findTestObject('POS Backoffice/Merchant Product Management/Create Category', [('BO-Silvi-Staging') : GlobalVariable.BOSilviStaging
                , ('merchantId') : GlobalVariable.merchantId, ('categoryName') : categoryProduct, ('bearerToken') : GlobalVariable.BOBearerToken]))

    def categoryResponse = slurper.parseText(jsonCategory.getResponseBodyContent())

    def categoryName = categoryResponse.data['category_name']

    def categoryId = categoryResponse.data['id']

    categoryMap.put(categoryName, categoryId)
}

for (int rowNum = 1; rowNum <= findTestData('Meneer').getRowNumbers(); rowNum++) {
    productCategory = findTestData('Meneer').getValue(1, rowNum)

    productName = findTestData('Meneer').getValue(2, rowNum)

    productSellingPrice = findTestData('Meneer').getValue(3, rowNum)

    productPurchasePrice = findTestData('Meneer').getValue(4, rowNum)

    productStock = findTestData('Meneer').getValue(5, rowNum)

    productMinStock = findTestData('Meneer').getValue(6, rowNum)

    productDescription = findTestData('Meneer').getValue(7, rowNum)

    String productCategoryId = categoryMap.get(productCategory)

    test = WS.sendRequest(findTestObject('POS Backoffice/Merchant Product Management/Create Product', [('BO-Silvi-Staging') : GlobalVariable.BOSilviStaging
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

