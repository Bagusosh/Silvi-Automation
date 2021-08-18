<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Payment Method</name>
   <tag></tag>
   <elementGuidId>e5d94202-b687-4490-83d8-cebd83c872f6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;code\&quot;:\&quot;VA-BNI\&quot;,\n    \&quot;name\&quot;:\&quot;Bank BCA\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-V2-Staging}/payment-methods/${paymentMethodId}</restUrl>
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
      <id>c35b7cd7-d650-42e1-9a05-ae1044bd86e4</id>
      <masked>false</masked>
      <name>Silvi-V2-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.paymentMethodId</defaultValue>
      <description></description>
      <id>b9926431-d3b4-4730-9f24-69f48ab2682c</id>
      <masked>false</masked>
      <name>paymentMethodId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
