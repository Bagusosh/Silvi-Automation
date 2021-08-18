<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Employee</name>
   <tag></tag>
   <elementGuidId>d8ec4289-be2e-46b4-8e7a-686d3a9e5caa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{   \r\n    \&quot;employee_name\&quot;: \&quot;Agi\&quot;,\r\n    \&quot;employee_image\&quot;: \&quot;\&quot;,\r\n    \&quot;employee_salary\&quot;: 1200000,\r\n    \&quot;employee_role\&quot;: \&quot;032ee074-2052-45d6-be47-9db9a0bfcb71\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Silvi-Staging}/merchant/${userId}/employee/${employeeId}</restUrl>
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
      <id>cc633f85-9207-435f-9a91-18a03bac958e</id>
      <masked>false</masked>
      <name>Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>5ad40378-0ca6-43e7-a321-221b3bcabbb2</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.employeeId</defaultValue>
      <description></description>
      <id>73428284-4642-4ac2-b025-57c7f95a7028</id>
      <masked>false</masked>
      <name>employeeId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
