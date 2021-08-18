<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login - Invalid</name>
   <tag></tag>
   <elementGuidId>77173393-34bd-40d1-b791-ee5accb92436</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;hp&quot;,
      &quot;value&quot;: &quot;${hp}&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;${password}&quot;
    },
    {
      &quot;name&quot;: &quot;tipeUser&quot;,
      &quot;value&quot;: &quot;merchant&quot;
    },
    {
      &quot;name&quot;: &quot;otp&quot;,
      &quot;value&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;loginKey&quot;,
      &quot;value&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
   </httpHeaderProperties>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseFlickStaging}/v1/users/auth/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseFlickStaging</defaultValue>
      <description></description>
      <id>616cbc83-5dc6-4f30-b276-746ca7d71bcb</id>
      <masked>false</masked>
      <name>BaseFlickStaging</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>ef7a5fd1-4c9c-42e4-8c70-21aa15b4513a</id>
      <masked>false</masked>
      <name>PIN</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>6397c5ae-20a3-48a7-bdcc-fdfa40cfe73e</id>
      <masked>false</masked>
      <name>hp</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.github.javafaker.Faker
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import groovy.json.JsonSlurper

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'meta.code', &quot;4010&quot;)


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
