<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Forgot-Password</name>
   <tag></tag>
   <elementGuidId>5db9ace9-bc7f-4d68-a413-5f43aec83ed6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;hp\&quot;: \&quot;082277831094\&quot;,\r\n    \&quot;newPassword\&quot;: \&quot;123456\&quot;,\r\n    \&quot;konfirmasiNewPassword\&quot;: \&quot;123456\&quot;,\r\n    \&quot;token\&quot;: \&quot;579894\&quot;,\r\n    \&quot;tipeUser\&quot;: \&quot;merchant\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v1/users/auth/reset-password</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseFlick-Staging</defaultValue>
      <description></description>
      <id>f11ef4fb-2b45-413b-b201-f808822614ee</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
