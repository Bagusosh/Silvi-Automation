<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Payment Pulsa</name>
   <tag></tag>
   <elementGuidId>0c0cd6fe-4e52-4aeb-b215-5746a2168cf7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;nomor\&quot; : \&quot;082277831094\&quot;,\r\n    \&quot;idProduk\&quot; : \&quot;1eccea09-6116-423f-9b8a-1ef35db4b673\&quot;,\r\n    \&quot;kodeProduk\&quot; : \&quot;1\&quot;,\r\n    \&quot;total\&quot; : \&quot;7200.00\&quot;,\r\n    \&quot;trackingId\&quot; : \&quot;8373795909082648730\&quot;,\r\n    \&quot;inquiryId\&quot; : \&quot;7824794564816158420\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseFlick-Staging}/v1/users/${userId}/payment/pulsa</restUrl>
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
      <id>92a9acbf-371e-4f92-bae8-c2aac1fb7a82</id>
      <masked>false</masked>
      <name>BaseFlick-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>84037472-95af-4e49-a009-ce192ea944da</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
