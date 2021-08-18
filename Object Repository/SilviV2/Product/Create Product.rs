<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Product</name>
   <tag></tag>
   <elementGuidId>828ffaab-78f3-4737-a1c5-68eb26945cac</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;${productName}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;description&quot;,
      &quot;value&quot;: &quot;${productDescription}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;category_id&quot;,
      &quot;value&quot;: &quot;${productCategoryId}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;active&quot;,
      &quot;value&quot;: &quot;${activeStatus}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;purchase_price&quot;,
      &quot;value&quot;: &quot;${productPurchasePrice}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;selling_price&quot;,
      &quot;value&quot;: &quot;${productSellingPrice}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
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
   <restUrl>${SilviV2Staging}/merchants/${merchantId}/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.SilviV2Staging</defaultValue>
      <description></description>
      <id>3175a372-a240-4076-8d0c-a9be552a3ef0</id>
      <masked>false</masked>
      <name>SilviV2Staging</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>14823d81-113f-4cd1-b63f-fd5c5111753b</id>
      <masked>false</masked>
      <name>productDescription</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e6f1324d-fd28-4516-a2c6-80760f8c6faa</id>
      <masked>false</masked>
      <name>productName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>58fc4871-bb59-4301-b16b-820e29c73b7f</id>
      <masked>false</masked>
      <name>productCategoryId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>df1848d7-9bd4-445c-979f-dfcb8cebe6a8</id>
      <masked>false</masked>
      <name>activeStatus</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>1bd948e2-b5a1-4381-96b5-162d5737c51e</id>
      <masked>false</masked>
      <name>productPurchasePrice</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>9f91b3ad-11e2-4cc2-9132-6277c92bdbe3</id>
      <masked>false</masked>
      <name>productSellingPrice</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.adminToken</defaultValue>
      <description></description>
      <id>4cf40a8f-989c-4ee2-b472-bad4ef7eb096</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantId</defaultValue>
      <description></description>
      <id>60be78fe-3752-4c56-9f37-0bf68ead9540</id>
      <masked>false</masked>
      <name>merchantId</name>
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
