<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Payment Method</name>
   <tag></tag>
   <elementGuidId>985a79b5-c2ce-4e07-a795-f5c83418fc73</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{    \r\n    \&quot;payment_name\&quot;: \&quot;Cash\&quot;,\r\n    \&quot;payment_image\&quot;: \&quot;\&quot;,\r\n    \&quot;payment_fee\&quot;: 0,\r\n    \&quot;payment_issuer\&quot;: \&quot;CASH-IT\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/payment-method/${paymentId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BO-Silvi-Staging</defaultValue>
      <description></description>
      <id>ab742974-b774-4053-849a-5951da232f7a</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.paymentId</defaultValue>
      <description></description>
      <id>2eb50a99-bb6a-4554-8af7-9718e1dc1397</id>
      <masked>false</masked>
      <name>paymentId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
