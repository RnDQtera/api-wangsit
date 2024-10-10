<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.019.01 Post Create Ticket Report Bug</name>
   <tag></tag>
   <elementGuidId>9c900de8-6764-4bc4-854c-15b1c934089b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;projectId\&quot;: \&quot;66fcbb2e2d7621db058ea2a0\&quot;,\n  \&quot;processProjectId\&quot;: \&quot;66e901c265b533fbe0c72c1a\&quot;,\n  \&quot;moduleId\&quot;: \&quot;66f4ce23352efef8e6ceca59\&quot;,\n  \&quot;subModuleId\&quot;: \&quot;66fa6f8829c27267c1b3b059\&quot;,\n  \&quot;taskId\&quot;: \&quot;66f287fdccfbaa1233f79051\&quot;,\n  \&quot;info\&quot;: {\n    \&quot;type\&quot;: \&quot;doc\&quot;,\n    \&quot;content\&quot;: [\n      {\n        \&quot;type\&quot;: \&quot;paragraph\&quot;,\n        \&quot;content\&quot;: [\n          {\n            \&quot;type\&quot;: \&quot;text\&quot;,\n            \&quot;text\&quot;: \&quot;Hello, this is a text example.\&quot;\n          }\n        ]\n      },\n      {\n        \&quot;type\&quot;: \&quot;paragraph\&quot;,\n        \&quot;content\&quot;: [\n          {\n            \&quot;type\&quot;: \&quot;text\&quot;,\n            \&quot;text\&quot;: \&quot;This text is bold.\&quot;,\n            \&quot;marks\&quot;: [\n              {\n                \&quot;type\&quot;: \&quot;bold\&quot;\n              }\n            ]\n          }\n        ]\n      },\n      {\n        \&quot;type\&quot;: \&quot;paragraph\&quot;,\n        \&quot;content\&quot;: [\n          {\n            \&quot;type\&quot;: \&quot;image\&quot;,\n            \&quot;attrs\&quot;: {\n              \&quot;src\&quot;: \&quot;https://example.com/image.jpg\&quot;,\n              \&quot;alt\&quot;: \&quot;Example Image\&quot;\n            }\n          }\n        ]\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f76a4749-908d-40ee-8e3a-92bd2a2d70ad</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9499f9e7-5133-4f30-8f9f-49db7fc6724e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b0e58668-1fe9-445e-996c-ef50e6cddea5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ticket}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'/ticket/report-bug'</defaultValue>
      <description></description>
      <id>ba3e3b48-d50b-4698-80f8-cf75dd316cfc</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d8e48ae2-64e2-49a1-a626-809c2b59c24a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ticket</defaultValue>
      <description></description>
      <id>052f2831-caf8-401e-a9c0-01d9481218dc</id>
      <masked>false</masked>
      <name>ticket</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>