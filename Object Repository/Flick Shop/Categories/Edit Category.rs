<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Category</name>
   <tag></tag>
   <elementGuidId>ae2cee2b-069c-4f28-98ac-1b490da69c7a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;categoryName\&quot; : \&quot;peralatan otomotif\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${FlickShop-Staging}/v1/product/category/${categoryId}
</restUrl>
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
      <id>9a84c7fe-3578-4543-b85b-874c4640c016</id>
      <masked>false</masked>
      <name>FlickShop-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryId</defaultValue>
      <description></description>
      <id>4498361e-a278-4209-9990-c65fc3e0b4f9</id>
      <masked>false</masked>
      <name>categoryId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
