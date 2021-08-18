<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Registration</name>
   <tag></tag>
   <elementGuidId>54a5bde9-2926-4c28-9fc8-88b38154eef0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseFlickStaging}/v1/users/register?hp=${hp}&amp;nama=${name}&amp;namaSesuaiKtp=${fullName}&amp;email=${email}&amp;password=${PIN}&amp;konfirmasiPassword=${PIN}&amp;type=${token_type}&amp;tipeMerchant=${merchant_type}&amp;kodeAgen=offline&amp;bankNoRekening&amp;bankNama&amp;bankAtasNama&amp;jenisUsaha=${business_type}&amp;lokasiUsaha=740dce0b-be84-4712-b06d-25525083af84&amp;detailLokasiUsaha=${address}&amp;referralCode&amp;token=${token}</restUrl>
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
      <id>b6a324fc-b752-4eb5-9aa2-1ddb4afd822f</id>
      <masked>false</masked>
      <name>BaseFlickStaging</name>
   </variables>
   <variables>
      <defaultValue>'kankurooo'</defaultValue>
      <description></description>
      <id>72ca25af-9489-4086-8f8e-5411f11aec26</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'kaaaaansiaa'</defaultValue>
      <description></description>
      <id>ed0a000e-eae6-4f11-a7e5-4fa8ab5cebde</id>
      <masked>false</masked>
      <name>fullName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6ab05b15-9f8f-4e13-a462-8130388afd6e</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>240cdf97-8c08-4a43-ae58-7cdd996f2dc2</id>
      <masked>false</masked>
      <name>PIN</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9833d6f0-9fa8-4290-afef-0300333a1e1b</id>
      <masked>false</masked>
      <name>token_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4a43f06d-46b4-4e57-b12e-176a801a763c</id>
      <masked>false</masked>
      <name>merchant_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b308d181-fb13-49a6-b259-1f66285aa0ef</id>
      <masked>false</masked>
      <name>business_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fef86515-5c4e-4d64-a89b-05f0be62abad</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bced349d-c661-48d4-b70d-3436f39710ca</id>
      <masked>false</masked>
      <name>token</name>
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
