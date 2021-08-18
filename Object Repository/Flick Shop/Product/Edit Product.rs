<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Product</name>
   <tag></tag>
   <elementGuidId>4013342d-d6c7-461e-b770-3aa79c5e7fca</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;name\&quot; : \&quot;\&quot;,\r\n    \&quot;categoryName\&quot; : \&quot;\&quot;,\r\n    \&quot;weight\&quot; : \&quot;\&quot;,\r\n    \&quot;quantity\&quot; : \&quot;\&quot;,\r\n    \&quot;price\&quot; : \&quot;\&quot;,\r\n    \&quot;description\&quot; : \&quot;\&quot;,\r\n    \&quot;minQuantity\&quot; : \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${FlickShop-Staging}/v1/product/${productId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.FlickShop-Staging</defaultValue>
      <description></description>
      <id>0e6a04b4-032d-418b-97e7-016d07c3a51b</id>
      <masked>false</masked>
      <name>FlickShop-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productId</defaultValue>
      <description></description>
      <id>5aa94cb6-b455-44ef-b603-f7deb650714c</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
