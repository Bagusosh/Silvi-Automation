<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Product</name>
   <tag></tag>
   <elementGuidId>4acf1e2c-88f2-4ddb-9795-176d35406887</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;product_name\&quot; : \&quot;${productName}\&quot;,\n\t\&quot;product_category\&quot; : \&quot;${productCategory}\&quot;,\n\t\&quot;product_selling_price\&quot; : ${sellingPrice},\n\t\&quot;product_purchase_price\&quot; : ${purchasePrice},\n\t\&quot;product_stock\&quot; : ${productStock},\n\t\&quot;product_min_stock\&quot; : ${productMinStock}\n}&quot;,
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
      <value>Bearer ${BearerToken}</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-Staging}merchant/${userId}/product</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.SilviStaging</defaultValue>
      <description></description>
      <id>ac62477a-861c-47d4-aa74-e04be126415f</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>baa9a512-b203-48fb-a0ac-c646f8ba62ca</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BearerToken</defaultValue>
      <description></description>
      <id>e72e0294-3709-4913-9519-c01cc8390cab</id>
      <masked>false</masked>
      <name>BearerToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c7c5ade4-eed1-487a-9069-575339ec55e5</id>
      <masked>false</masked>
      <name>productName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>71eb6d5d-1f9a-419e-b2f8-2593c175ed5c</id>
      <masked>false</masked>
      <name>productCategory</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>b8f75677-7b51-4359-8a65-d52a140cc269</id>
      <masked>false</masked>
      <name>sellingPrice</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>be0d1214-261b-4ae2-83f6-fe411d2c685e</id>
      <masked>false</masked>
      <name>productStock</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>9b1c2a8d-8101-4c60-8f6b-16e4e5526874</id>
      <masked>false</masked>
      <name>purchasePrice</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>24871790-5734-4957-a8e4-6ecd68bf59d3</id>
      <masked>false</masked>
      <name>productMinStock</name>
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
