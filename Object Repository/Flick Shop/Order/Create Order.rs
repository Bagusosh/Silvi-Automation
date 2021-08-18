<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>d325567a-63b8-46ed-ba3b-7c6aa682b2a9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;orders\&quot;: [\r\n    {\r\n      \&quot;orderDetails\&quot;: [\r\n        {\r\n          \&quot;productId\&quot;: \&quot;1eb92600-a616-4a3a-a5cc-ffba1dcd8dff\&quot;,\r\n           //The first index being treated as parent and the next index is being treated as children.\r\n          \&quot;variantItemIds\&quot;: [\r\n            \&quot;\&quot;\r\n          ],\r\n          \&quot;quantity\&quot;: 1\r\n        }\r\n      ],\r\n      \&quot;shippingProviderCode\&quot;: \&quot;02\&quot;,\r\n      \&quot;shippingServiceType\&quot;: \&quot;\&quot;,\r\n      // discount is optional. it must be uuid\r\n      \&quot;discountId\&quot;: \&quot;\&quot;\r\n    }\r\n  ],\r\n  \&quot;addressId\&quot;: \&quot;e855f4c5-5a7e-4e5b-ae01-d6e8045b6800\&quot;,\r\n  //paymentCode is respecting payment code which fullfill \r\n  \&quot;paymentCode\&quot;: \&quot;string\&quot;,\r\n  //flickPin is pin for accessing flick account to authorize user. only needed when paymentCode value is FLICK\r\n  \&quot;flickPin\&quot;: \&quot;string\&quot;,\r\n  // The cc is optional and only being needed when paymentCode property is CC\r\n  \&quot;cc\&quot;: {\r\n    // instmnt_mon is n month of installment \r\n    \&quot;instmnt_mon\&quot;: 0,\r\n    // instmnt_type is the type of installment which consist of \r\n    // - 0 : installment fully pay the order\r\n    // - 1 : installment is payed recurringly according to instment_mon and recurr_opt\r\n    \&quot;instmnt_type\&quot;: 0,\r\n    // recurr_opt is installment which being payed every x date on every month\r\n    \&quot;recurr_opt\&quot;: 0\r\n  }\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${FlickShop-Staging}/v1/order</restUrl>
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
      <id>8cb8a2a8-4e02-4073-99d1-9e00d2205681</id>
      <masked>false</masked>
      <name>FlickShop-Staging</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
