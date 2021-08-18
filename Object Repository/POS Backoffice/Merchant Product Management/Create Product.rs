<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Product</name>
   <tag></tag>
   <elementGuidId>64e49388-02ef-4305-8668-6feaaf0d13c9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;product_name\&quot; : \&quot;${productName}\&quot;,\n\t\&quot;product_category\&quot; : \&quot;${productCategoryId}\&quot;,\n\t\&quot;product_selling_price\&quot; : ${productSellingPrice},\n\t\&quot;product_purchase_price\&quot; : ${productPurchasePrice},\n\t\&quot;product_stock\&quot; : ${productStock},\n\t\&quot;product_min_stock\&quot; : ${productMinStock},\n    \&quot;product_description\&quot; : \&quot;${productDescription}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BOBaseSilvi}/backoffice/merchant/${merchantId}/product</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BOSilviProd</defaultValue>
      <description></description>
      <id>cb5fbbf5-2db5-4e40-a34f-d3206b407c52</id>
      <masked>false</masked>
      <name>BOBaseSilvi</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantId</defaultValue>
      <description></description>
      <id>9e23def4-0ffd-44cb-8967-5bdc39426dde</id>
      <masked>false</masked>
      <name>merchantId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BOBearerToken</defaultValue>
      <description></description>
      <id>40ed7548-909f-4388-a113-2adba31083a4</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>214267d3-78a8-4ab4-899e-8128b1d37c4a</id>
      <masked>false</masked>
      <name>productName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dcd1ac05-d184-41f1-9003-e0bdd56bdf8b</id>
      <masked>false</masked>
      <name>productCategoryId</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>bd2cbc46-8a0d-45a4-8816-912993959179</id>
      <masked>false</masked>
      <name>productSellingPrice</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>cfdd04c8-fea5-4e73-b2c5-51237373b3d9</id>
      <masked>false</masked>
      <name>productPurchasePrice</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>af99ca3e-4c04-41d7-85db-3f5d7d0dcf3b</id>
      <masked>false</masked>
      <name>productStock</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>95834136-04a9-4fb8-bab8-38eec4cb0dd0</id>
      <masked>false</masked>
      <name>productMinStock</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>15ba9170-bb8f-43b6-9118-19872659b0ea</id>
      <masked>false</masked>
      <name>productDescription</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
