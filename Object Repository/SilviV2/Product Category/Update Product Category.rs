<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Product Category</name>
   <tag></tag>
   <elementGuidId>86adcdeb-01cc-4812-9ede-fb481edfb289</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Minuman\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-V2-Staging}/product-categories/${categoryId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Staging</defaultValue>
      <description></description>
      <id>f3e4bbb0-6f50-4ea4-a173-7fbdf2dcd7ab</id>
      <masked>false</masked>
      <name>Silvi-V2-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryId</defaultValue>
      <description></description>
      <id>1c20fab6-88f2-4e27-a09a-7afc5b42ea85</id>
      <masked>false</masked>
      <name>categoryId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
