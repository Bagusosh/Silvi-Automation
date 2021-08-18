<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Discount Merchant</name>
   <tag></tag>
   <elementGuidId>c761a475-f7a9-425e-8136-2c926e580151</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;type\&quot;: \&quot;order\&quot;,\n    \&quot;amount\&quot;: 10000,\n    \&quot;unit\&quot;: \&quot;rupiah\&quot;,\n    \&quot;start_date\&quot;:\&quot;2021-05-03T00:00:00Z\&quot;,\n    \&quot;end_date\&quot;:\&quot;2021-07-03T00:00:00Z\&quot;,\n    \&quot;merchant_id\&quot;: \&quot;c010c7ed-527f-4514-8ff3-0aeb7acf37c0\&quot;,\n    \&quot;minimum_value\&quot;: 50000\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Silvi-V2-Local}/discount-merchants</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-V2-Local</defaultValue>
      <description></description>
      <id>1823c114-0a22-4856-80b8-cd7321622518</id>
      <masked>false</masked>
      <name>Silvi-V2-Local</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
