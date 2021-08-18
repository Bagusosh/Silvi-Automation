<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Accept Acquisition</name>
   <tag></tag>
   <elementGuidId>f031d292-e92f-4ec2-a2a0-f6bca0c8753e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{        \r\n        \&quot;merchant_table_amount\&quot;: 13,\r\n        \&quot;acquisition_form_address\&quot;: \&quot;Jalan Harapan No.3 Malayka Resident No.5 RT.3/RW.3, Bintaro Kec. Pesanggrahan Kota Jakarta Selatan Daerah Khusus Ibukota Jakarta Kode Pos 12330 (021) 29866319\&quot;,\r\n        \&quot;acquisition_form_detail\&quot;: \&quot;\&quot;,\r\n        \&quot;acquisition_form_subscription\&quot;: \&quot;35ec44bd-8d3b-4e3a-a217-4002b8ad1c49\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/acquisition/${acquisitionId}/accept</restUrl>
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
      <id>b7d84e0e-5404-48d7-9ce2-b8b9b0714226</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.acquisitionId</defaultValue>
      <description></description>
      <id>97bd539d-385a-480e-bd91-b03b0ae9f88e</id>
      <masked>false</masked>
      <name>acquisitionId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
