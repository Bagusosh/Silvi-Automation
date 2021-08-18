<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Complete Order</name>
   <tag></tag>
   <elementGuidId>4465a072-fe71-43f6-a32d-714ee417c57b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\t\&quot;money_recieved\&quot; : 500000\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}/merchant/${userId}/order/${orderId}/complete?billing_type=INVOICE</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Silvi-Staging</defaultValue>
      <description></description>
      <id>43e2ee73-deed-44f9-8b54-2466217d6120</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>ea542fef-440c-400c-814e-9ca6b66ae18d</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.orderId</defaultValue>
      <description></description>
      <id>e0068a97-b0f7-461a-ab29-d94204ea87e5</id>
      <masked>false</masked>
      <name>orderId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
